#!/usr/bin/env python3
"""Check generated Android package identity, ABIs, and native entry points."""

import argparse
from pathlib import Path
import re
import subprocess
import tempfile
import zipfile


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("apk", type=Path)
    parser.add_argument("--package", required=True)
    parser.add_argument("--abis", default="arm64-v8a")
    parser.add_argument("--build-tools", type=Path, required=True)
    parser.add_argument("--readelf", type=Path, required=True)
    args = parser.parse_args()
    badging = subprocess.check_output(
        [str(args.build_tools / "aapt"), "dump", "badging", str(args.apk)], text=True
    )
    if f"package: name='{args.package}'" not in badging:
        raise SystemExit("APK application id differs from the generated project")
    if "launchable-activity: name='android.app.NativeActivity'" not in badging:
        raise SystemExit("APK does not launch NativeActivity")
    subprocess.run(
        [str(args.build_tools / "apksigner"), "verify", str(args.apk)], check=True
    )
    expected = set(args.abis.split(","))
    with zipfile.ZipFile(args.apk) as archive, tempfile.TemporaryDirectory() as scratch:
        libraries = [
            name
            for name in archive.namelist()
            if re.fullmatch(r"lib/[^/]+/libmirui_android\.so", name)
        ]
        if {name.split("/")[1] for name in libraries} != expected:
            raise SystemExit("APK native libraries do not match the requested ABIs")
        for name in libraries:
            path = Path(scratch) / "libmirui_android.so"
            path.write_bytes(archive.read(name))
            symbols = subprocess.check_output(
                [str(args.readelf), "--dyn-syms", "--wide", str(path)], text=True
            )
            if not re.search(
                r"GLOBAL\s+DEFAULT\s+\d+\s+ANativeActivity_onCreate\b", symbols
            ):
                raise SystemExit(f"{name}: missing NativeActivity entry point")
    print(f"Android APK verified: {args.package}, {', '.join(sorted(expected))}")


if __name__ == "__main__":
    main()
