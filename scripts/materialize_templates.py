#!/usr/bin/env python3
"""Materialize standalone and workspace targets from canonical adapters."""

from __future__ import annotations

import argparse
import difflib
import os
from dataclasses import dataclass
from pathlib import Path
import tempfile


ROOT = Path(__file__).resolve().parents[1]
COMPONENTS = ROOT / "components"
TEMPLATES = ROOT / "templates"
RELEASE_PROFILE = """[profile.release]
opt-level = "s"
lto = "fat"
codegen-units = 1
strip = true"""

WASM_RELEASE_PROFILE = """[profile.release]
opt-level = "z"
lto = true
codegen-units = 1
strip = true
panic = "abort"
"""


@dataclass(frozen=True)
class Target:
    name: str
    standalone_dir: str
    workspace_package: str
    entry_name: str
    release_profile: str = RELEASE_PROFILE


TARGETS = (
    Target("desktop", "sdl-only", "desktop", "main.rs"),
    Target("wasm", "wasm", "wasm", "lib.rs", WASM_RELEASE_PROFILE),
    Target("esp32c3", "esp32c3", "target-esp32c3", "main.rs"),
    Target("android", "android", "target-android", "lib.rs"),
    Target("ios", "ios", "target-ios", "lib.rs"),
)


def render(source: Path, replacements: dict[str, str]) -> bytes:
    text = source.read_text(encoding="utf-8")
    for key, value in replacements.items():
        text = text.replace(f"@@{key}@@", value)
    unresolved = [part.split("@@", 1)[0] for part in text.split("@@")[1::2]]
    if unresolved:
        raise ValueError(f"unresolved materializer tokens in {source}: {unresolved}")
    return text.rstrip().encode() + b"\n"


def planned_files() -> dict[Path, bytes]:
    files: dict[Path, bytes] = {}
    standalone_ui = render(COMPONENTS / "shared" / "ui.rs.in", {"CRATE_ATTR": ""})
    workspace_ui = render(
        COMPONENTS / "shared" / "ui.rs.in",
        {"CRATE_ATTR": '#![cfg_attr(not(feature = "std"), no_std)]\n'},
    )

    for target in TARGETS:
        source_dir = COMPONENTS / "targets" / target.name
        standalone = TEMPLATES / target.standalone_dir
        workspace = TEMPLATES / "workspace" / "targets" / target.name

        standalone_replacements = {
            "PACKAGE_NAME": "{{project-name}}",
            "APP_DEPENDENCY": "",
            "APP_IMPORT": '#[path = "ui.rs"]\nmod template_app;',
            "BACKEND_VAR": "backend",
            "RELEASE_PROFILE": target.release_profile,
        }
        workspace_replacements = {
            "PACKAGE_NAME": target.workspace_package,
            "APP_DEPENDENCY": 'app = { path = "../../app", features = ["std"] }'
            if target.name != "esp32c3"
            else 'app = { path = "../../app", default-features = false }',
            "APP_IMPORT": "use app as template_app;",
            "BACKEND_VAR": f"{target.name}-backend",
            "RELEASE_PROFILE": "",
        }

        files[standalone / "Cargo.toml"] = render(
            source_dir / "Cargo.toml.in", standalone_replacements
        )
        files[standalone / "src" / target.entry_name] = render(
            source_dir / f"{target.entry_name}.in", standalone_replacements
        )
        files[standalone / "src" / "ui.rs"] = standalone_ui

        files[workspace / "Cargo.toml"] = render(
            source_dir / "Cargo.toml.in", workspace_replacements
        )
        files[workspace / "src" / target.entry_name] = render(
            source_dir / f"{target.entry_name}.in", workspace_replacements
        )

    files[TEMPLATES / "workspace" / "app" / "src" / "lib.rs"] = workspace_ui
    files[TEMPLATES / "workspace" / "app" / "Cargo.toml"] = render(
        COMPONENTS / "shared" / "app.Cargo.toml.in",
        {},
    )

    copied_platform_files = {
        "wasm": ("Trunk.toml", "index.html"),
        "esp32c3": (".cargo/config.toml", "rust-toolchain.toml"),
        "ios": (
            "ios/Info.plist",
            "ios/main.m",
            "ios/{{project-name}}.xcodeproj/project.pbxproj",
        ),
    }
    for target_name, relative_paths in copied_platform_files.items():
        standalone_dir = next(
            target.standalone_dir for target in TARGETS if target.name == target_name
        )
        for relative_path in relative_paths:
            source = TEMPLATES / standalone_dir / relative_path
            data = source.read_bytes()
            if target_name == "ios" and relative_path.endswith("project.pbxproj"):
                data = data.replace(b"lib{{crate_name}}.a", b"libtarget_ios.a")
                data = data.replace(b"-l{{crate_name}}", b"-ltarget_ios")
                data = data.replace(
                    b"$(SRCROOT)/../target/", b"$(SRCROOT)/../../../target/"
                )
            destination = TEMPLATES / "workspace" / "targets" / target_name / relative_path
            files[destination] = data
    return files


def atomic_write(path: Path, data: bytes) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    with tempfile.NamedTemporaryFile(dir=path.parent, delete=False) as handle:
        handle.write(data)
        temporary = Path(handle.name)
    os.replace(temporary, path)


def write() -> int:
    for path, data in sorted(planned_files().items()):
        if not path.exists() or path.read_bytes() != data:
            atomic_write(path, data)
            print(path.relative_to(ROOT))
    return 0


def check() -> int:
    failed = False
    for path, expected in sorted(planned_files().items()):
        actual = path.read_bytes() if path.exists() else b""
        if actual == expected:
            continue
        failed = True
        print(f"out of date: {path.relative_to(ROOT)}")
        actual_text = actual.decode("utf-8", errors="replace").splitlines()
        expected_text = expected.decode("utf-8").splitlines()
        for line in difflib.unified_diff(
            actual_text,
            expected_text,
            fromfile=str(path.relative_to(ROOT)),
            tofile="materialized",
            lineterm="",
        ):
            print(line)
    return 1 if failed else 0


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("mode", choices=("write", "check"))
    args = parser.parse_args()
    return write() if args.mode == "write" else check()


if __name__ == "__main__":
    raise SystemExit(main())
