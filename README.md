# mirui-templates

[![CI](https://github.com/W-Mai/mirui-templates/actions/workflows/ci.yml/badge.svg)](https://github.com/W-Mai/mirui-templates/actions/workflows/ci.yml)

[cargo-generate](https://github.com/cargo-generate/cargo-generate) templates for [mirui](https://github.com/W-Mai/mirui), a `no_std` ECS-driven UI framework for embedded, mobile, desktop, Linux, and WebAssembly targets.

## Install cargo-generate

```bash
cargo install cargo-generate
```

## Available templates

| Template | Description |
|----------|-------------|
| `sdl-only` | Single-crate SDL desktop project. ~5 minutes from zero to a window. |
| `esp32c3` | Single-crate ESP32-C3 embedded project, framebuffer-driven, esp-hal stack. |
| `workspace` | Cargo workspace with a shared UI library and desktop, browser, and ESP32-C3 target crates. Globs in `targets/*` so adding another target does not require editing the member list. |
| `wasm` | Browser target on the `web-canvas` (Canvas 2D) Surface backend. Builds and serves with [trunk](https://trunkrs.dev). |
| `android` | Android NativeActivity project with a generation-time choice between direct WGPU and software rasterization. |
| `ios` | iPhone and iPad Xcode project with a generation-time choice between direct WGPU and software rasterization. |

## Backend and target coverage

| Runtime | Template | mirui feature | Verification |
|---------|----------|---------------|--------------|
| SDL software | `sdl-only`, `workspace/targets/desktop` | `sdl` | CI build |
| Browser Canvas 2D | `wasm`, `workspace/targets/wasm` | `web-canvas` | CI WebAssembly build |
| Android WGPU | `android` | `wgpu-android` | CI AArch64 APK build |
| Android software | `android` | `wgpu-android` | CI AArch64 APK build |
| iOS WGPU | `ios` | `wgpu` | CI unsigned arm64 simulator app build |
| iOS software | `ios` | `wgpu` | CI unsigned arm64 simulator app build |
| ESP32-C3 framebuffer | `esp32c3`, `workspace/targets/esp32c3` | no backend feature | RISC-V cross-build; complete ST7735S wiring is verified in `mirui-examples` |

mirui also exposes `sdl-gpu`, `linux-fb`, `linux-drm`, and `nuttx`. These backends do not have dedicated generator templates in this repository. ESP32-S3, RP2040, STM32, and other MCUs can reuse the shared `no_std` app structure, but need target-specific allocator, clock, input, framebuffer flush, and panel code.

The ESP32-C3 templates keep antialiasing and profiling explicit: enable `quad-aa` for transformed-edge quality or `perf` for frame timing. Both stay off by default so a generated firmware starts with the smaller embedded configuration.

## Use a template

```bash
cargo generate W-Mai/mirui-templates sdl-only --name hello-mirui
cd hello-mirui
cargo run
```

Each template prompts for a project name and the mirui version to pin. Mobile templates also prompt for a rendering path and application identifier. The generated project's `Cargo.toml` and `README.md` are filled in from the prompts.

## Pinning to a mirui release

The templates default to whichever mirui version is current when this repo's `main` was last bumped (see the per-template `cargo-generate.toml` default). Pass `--define mirui-version=0.X` to override.

## Maintenance

This repo tracks the [mirui](https://github.com/W-Mai/mirui) release cycle. When mirui ships a new minor, `cargo xtask templates-bump` inside the mirui repo updates the per-template `cargo-generate.toml` default and commits the bump here.

The mirui version that ends up in a generated project's `Cargo.toml` comes from the cargo-generate prompt at generation time, not from the template files. `--define mirui-version=0.45` overrides the template default.

## Contributing

To propose a new template, such as an additional MCU or a different desktop backend:

1. Add a `templates/<name>/` directory using the same layout as an existing template, including a per-template `cargo-generate.toml` declaring the placeholders.
2. Verify locally:
   ```bash
   cargo generate --path templates/<name> --name testfoo --define mirui-version=0.45
   cd testfoo && cargo build  # or cargo build --release for embedded
   ```
3. Add a job to `.github/workflows/ci.yml` that runs the same `cargo generate` + `cargo build` so CI catches regressions.
4. Update `README.md` and `CHANGELOG.md` with an entry for the new template.

For bug fixes to existing templates, file an issue with the exact `cargo generate` invocation, the cargo error, and your toolchain versions (`rustc -V`, `cargo -V`, `cargo generate -V`).

## See also

- [mirui](https://github.com/W-Mai/mirui) — the framework itself.
- [mirui-examples](https://github.com/W-Mai/mirui-examples) — hardware demos including the ST7735S board file used by the `esp32c3` template.

## License

MIT — see `LICENSE`.
