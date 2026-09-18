#!/usr/bin/env python3

from pathlib import Path
import re


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


def main() -> None:
    versions = {
        name: mirui_version(TEMPLATES / name / "cargo-generate.toml")
        for name in ("esp32c3", "sdl-only", "wasm", "workspace")
    }
    if len(set(versions.values())) != 1:
        details = ", ".join(f"{name}={version}" for name, version in versions.items())
        raise SystemExit(f"mirui-version defaults differ: {details}")

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
