# mirui-templates

[![CI](https://github.com/W-Mai/mirui-templates/actions/workflows/ci.yml/badge.svg)](https://github.com/W-Mai/mirui-templates/actions/workflows/ci.yml)

[cargo-generate](https://github.com/cargo-generate/cargo-generate)
templates for [mirui](https://github.com/W-Mai/mirui), a `no_std`
ECS-driven UI framework for embedded, desktop, and WebAssembly.

## Install cargo-generate

```bash
cargo install cargo-generate
```

## Available templates

| Template | Description |
|----------|-------------|
| `sdl-only` | Single-crate SDL desktop project. ~5 minutes from zero to a window. |
| `esp32c3` | Single-crate ESP32-C3 embedded project, framebuffer-driven, esp-hal stack. |
| `workspace` | Cargo workspace with a shared UI library and one binary crate per target (`targets/desktop`, `targets/esp32c3`). Globs in `targets/*` so adding a new MCU is `cp -r` plus four edits. |
| `wasm` | Stub for an upcoming `web-canvas` Surface backend. **Does not build yet** — see the template's README for the timeline. |

## Use a template

```bash
cargo generate W-Mai/mirui-templates sdl-only --name hello-mirui
cd hello-mirui
cargo run
```

Each template prompts for a project name and the mirui version to pin.
The generated project's `Cargo.toml` and `README.md` are filled in
from the prompts.

## Pinning to a mirui release

The templates default to whichever mirui version is current when this
repo's `main` was last bumped (see the per-template `cargo-generate.toml`
default). Pass `--define mirui-version=0.X` to override.

## Maintenance

This repo tracks the [mirui](https://github.com/W-Mai/mirui) release
cycle. When mirui ships a new minor, `cargo xtask templates-bump`
inside the mirui repo updates the version pins and pushes a matching
commit here.

## License

MIT — see `LICENSE`.
