#!/usr/bin/env python3

from pathlib import Path
import json
import os
import re
import subprocess
import sys
import tempfile
import xml.etree.ElementTree as ET

from materialize_templates import check as check_materialized


ROOT = Path(__file__).resolve().parent.parent
TEMPLATES = ROOT / "templates"


def section(path: Path, name: str) -> tuple[str, ...]:
    lines = path.read_text().splitlines()
    header = f"[{name}]"
    try:
        start = lines.index(header) + 1
    except ValueError as error:
        raise SystemExit(f"{path}: missing {header}") from error

    values = []
    for line in lines[start:]:
        stripped = line.strip()
        if stripped.startswith("["):
            break
        if stripped and not stripped.startswith("#"):
            values.append(stripped)
    return tuple(values)


def mirui_version(path: Path) -> str:
    match = re.search(r'^default = "([^"]+)"$', path.read_text(), re.MULTILINE)
    if match is None:
        raise SystemExit(f"{path}: missing mirui-version default")
    return match.group(1)


def require_equal(label: str, left: object, right: object) -> None:
    if left != right:
        raise SystemExit(f"{label} differs between ESP32-C3 templates")


def check_ios_build() -> None:
    with tempfile.TemporaryDirectory(prefix="mirui ios contract ") as temporary:
        root = Path(temporary)
        stub = root / "bin/cargo"
        stub.parent.mkdir()
        stub.write_text(
            f"#!{sys.executable}\n"
            "import argparse, json, os, sys\n"
            "from pathlib import Path\n"
            "if os.environ.get('MIRUI_STUB_EXIT'): sys.exit(int(os.environ['MIRUI_STUB_EXIT']))\n"
            "parser = argparse.ArgumentParser()\n"
            "parser.add_argument('command', choices=['build'])\n"
            "for name in ('profile', 'target', 'target-dir'):\n"
            "    parser.add_argument('--' + name, required=True)\n"
            "args = parser.parse_args()\n"
            "profile = 'debug' if args.profile == 'dev' else args.profile\n"
            "artifact = Path(args.target_dir) / args.target / profile / os.environ['MIRUI_STUB_LIBRARY']\n"
            "artifact.parent.mkdir(parents=True, exist_ok=True)\n"
            "artifact.touch()\n"
            "Path(os.environ['MIRUI_STUB_RESULT']).write_text(json.dumps({'cwd': os.getcwd()}))\n"
        )
        stub.chmod(0o755)
        for relative, library, project_root in (
            ("ios", "libsample_app.a", root / "standalone"),
            ("workspace/targets/ios", "libtarget_ios.a", root / "workspace"),
        ):
            project = (
                TEMPLATES / relative / "ios/{{project-name}}.xcodeproj/project.pbxproj"
            )
            source = project.read_text().replace("{{crate_name}}", "sample_app")
            script = json.loads(
                re.search(r'shellScript = ("(?:\\.|[^"\\])*");', source)[1]
            )
            output = json.loads(
                re.search(r'outputPaths = \(("[^"\n]+"),\);', source)[1]
            )
            configurations = re.findall(
                r"buildSettings = \{(.*?)\};\s*name = (\w+);", source, re.DOTALL
            )
            checked = set()
            for block, configuration in configurations:
                settings = {
                    key.strip('"'): json.loads(value)
                    if value.startswith('"')
                    else value.strip()
                    for key, value in re.findall(
                        r'("[^"\n]+"|\w+)\s*=\s*("(?:\\.|[^"\\])*"|[^;\n]+);', block
                    )
                }
                if "MIRUI_RUST_PROFILE" not in settings:
                    continue
                expected_profile = {"Debug": "dev", "Release": "release"}[configuration]
                if settings["MIRUI_RUST_PROFILE"] != expected_profile:
                    raise SystemExit(
                        f"{project}: {configuration} uses the wrong Cargo profile"
                    )
                for sdk, target in (
                    ("iphoneos", "aarch64-apple-ios"),
                    ("iphonesimulator", "aarch64-apple-ios-sim"),
                ):
                    checked.add((configuration, sdk))
                    srcroot = project_root / (
                        "targets/ios/ios"
                        if relative.startswith("workspace/")
                        else "ios"
                    )
                    srcroot.mkdir(parents=True, exist_ok=True)
                    settings["SRCROOT"] = str(srcroot)
                    settings["MIRUI_RUST_TARGET"] = settings[
                        f"MIRUI_RUST_TARGET[sdk={sdk}*]"
                    ]
                    if settings["MIRUI_RUST_TARGET"] != target:
                        raise SystemExit(f"{project}: {sdk} uses the wrong Rust target")

                    def expand(value: str) -> str:
                        return re.sub(
                            r"\$\((\w+)\)",
                            lambda match: expand(settings[match[1]]),
                            value,
                        )

                    result = root / "cargo-result.json"
                    environment = dict(os.environ)
                    environment.update(
                        {
                            key: expand(value)
                            for key, value in settings.items()
                            if key == "SRCROOT"
                            or (key.startswith("MIRUI_") and "[" not in key)
                        }
                    )
                    environment.update(
                        PATH=str(stub.parent) + os.pathsep + environment["PATH"],
                        CARGO_TARGET_DIR=str(root / "unrelated-target"),
                        MIRUI_STUB_LIBRARY=library,
                        MIRUI_STUB_RESULT=str(result),
                    )
                    subprocess.run(
                        ["/bin/sh", "-c", script], env=environment, check=True
                    )
                    profile_directory = (
                        "debug" if expected_profile == "dev" else "release"
                    )
                    artifact = (
                        project_root / "target" / target / profile_directory / library
                    )
                    if (
                        not artifact.is_file()
                        or Path(expand(output)).resolve() != artifact.resolve()
                        or Path(expand(settings["LIBRARY_SEARCH_PATHS"])).resolve()
                        != artifact.parent.resolve()
                        or Path(json.loads(result.read_text())["cwd"]).resolve()
                        != srcroot.parent.resolve()
                    ):
                        raise SystemExit(
                            f"{project}: {configuration}/{sdk} Rust output and linker paths differ"
                        )
            if checked != {
                (configuration, sdk)
                for configuration in ("Debug", "Release")
                for sdk in ("iphoneos", "iphonesimulator")
            }:
                raise SystemExit(f"{project}: missing iOS build configuration")
            failed_build = subprocess.run(
                ["/bin/sh", "-c", script],
                env=dict(environment, MIRUI_STUB_EXIT="7"),
                capture_output=True,
            )
            if failed_build.returncode != 7:
                raise SystemExit(f"{project}: Cargo failure does not fail the build")
            result.unlink()
            missing_directory = subprocess.run(
                ["/bin/sh", "-c", script],
                env=dict(environment, SRCROOT=str(root / "missing/ios")),
                capture_output=True,
            )
            if missing_directory.returncode == 0 or result.exists():
                raise SystemExit(
                    f"{project}: Cargo ran after changing directory failed"
                )


def main() -> None:
    if check_materialized() != 0:
        raise SystemExit("materialized templates are out of date")

    check_ios_build()

    for android in (TEMPLATES / "android", TEMPLATES / "workspace/targets/android"):
        manifest = ET.parse(android / "android/src/main/AndroidManifest.xml")
        attr = "{http://schemas.android.com/apk/res/android}"
        activity = manifest.find("application/activity")
        if (
            activity is None
            or activity.get(attr + "name") != "android.app.NativeActivity"
        ):
            raise SystemExit(f"{android}: missing NativeActivity")
        library = activity.find("meta-data")
        if library is None or library.get(attr + "value") != "mirui_android":
            raise SystemExit(
                f"{android}: manifest does not select the Cargo native library"
            )
        if 'name = "mirui_android"' not in (android / "Cargo.toml").read_text():
            raise SystemExit(
                f"{android}: native library name differs from the manifest"
            )
        if "package.metadata.android" in (android / "Cargo.toml").read_text():
            raise SystemExit(f"{android}: duplicate cargo-apk manifest configuration")
        if not (android / "android/gradlew").stat().st_mode & 0o111:
            raise SystemExit(f"{android}: Gradle wrapper is not executable")

    versions = {
        name: mirui_version(TEMPLATES / name / "cargo-generate.toml")
        for name in ("android", "desktop", "esp32c3", "ios", "wasm", "workspace")
    }
    if len(set(versions.values())) != 1:
        details = ", ".join(f"{name}={version}" for name, version in versions.items())
        raise SystemExit(f"mirui-version defaults differ: {details}")

    manifest_sources = tuple((ROOT / "components").rglob("Cargo.toml.in")) + tuple(
        TEMPLATES.rglob("Cargo.toml")
    )
    for manifest in manifest_sources:
        text = manifest.read_text()
        if "mirui = {" not in text:
            continue
        if 'git = "https://github.com/W-Mai/mirui"' in text or "MIRUI_REV" in text:
            raise SystemExit(
                f"{manifest}: mirui must resolve from the selected published version"
            )

    standalone_dir = TEMPLATES / "esp32c3"
    workspace_dir = TEMPLATES / "workspace" / "targets" / "esp32c3"
    standalone_manifest = standalone_dir / "Cargo.toml"
    workspace_manifest = workspace_dir / "Cargo.toml"

    require_equal(
        "Cargo features",
        section(standalone_manifest, "features"),
        section(workspace_manifest, "features"),
    )

    workspace_dependencies = tuple(
        line
        for line in section(workspace_manifest, "dependencies")
        if not line.startswith("app = ")
    )
    require_equal(
        "embedded dependencies",
        section(standalone_manifest, "dependencies"),
        workspace_dependencies,
    )
    require_equal(
        "Rust toolchain",
        (standalone_dir / "rust-toolchain.toml").read_bytes(),
        (workspace_dir / "rust-toolchain.toml").read_bytes(),
    )
    require_equal(
        "Cargo target configuration",
        (standalone_dir / ".cargo" / "config.toml").read_bytes(),
        (workspace_dir / ".cargo" / "config.toml").read_bytes(),
    )


if __name__ == "__main__":
    main()
