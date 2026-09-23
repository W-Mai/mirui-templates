# Changelog

All notable changes to mirui-templates are documented here. Versions
track the [mirui](https://github.com/W-Mai/mirui) release cycle: when
mirui ships a new minor, `cargo xtask templates-bump` inside the mirui
repo updates the per-template `cargo-generate.toml` defaults and lands
a matching commit here.

The format follows [Keep a Changelog](https://keepachangelog.com/en/1.1.0/).

## [Unreleased]

### Added

- `desktop` template and workspace desktop target support WGPU or SDL selection during project generation.
- Android and iOS templates support direct WGPU rendering or software rasterization selected during project generation.
- Mobile templates retain application state across native lifecycle suspension and resume, preserve touch input, and bound software framebuffer allocation.
- The workspace template includes Android and iOS target crates that share the same UI library as desktop, WebAssembly, and ESP32-C3.
- Successful mobile CI builds publish test-signed Android APKs and unsigned arm64 iOS Simulator archives as downloadable release assets.
- The shared starter UI provides responsive buttons, a live slider, and reactive status text across every generated target.

### Changed

- CI keeps mirui version defaults aligned across templates and rejects divergent features, dependencies, toolchains, or target configuration between the standalone and workspace ESP32-C3 starters.
- Canonical target adapters materialize standalone and workspace variants deterministically, and CI rejects generated-file drift.
- Mobile software templates use native device density within a 32 MiB default framebuffer budget, and generated roots respect platform safe areas.
- Generated manifests use the prompted published mirui version; generated applications can commit `Cargo.lock` to preserve their resolved dependency graph.
- SDL desktop projects compile and statically link SDL2 without host-specific library search paths. Generated Cargo configuration supports SDL2's build with CMake 4.
- ESP32-C3 framebuffer callbacks receive the clipped `PhysicalRect` value required by mirui 0.44.
- ESP32-C3 templates expose and cross-build `quad-aa` and `perf` as opt-in Cargo features instead of forcing transformed-quad antialiasing into every firmware.
- README target matrices distinguish CI-built templates, hardware-verified examples, available mirui backends, and unverified MCU extension points.
- `wasm` template now builds and serves with [trunk](https://trunkrs.dev):
  `index.html` carries a `<link data-trunk rel="rust">`, a `Trunk.toml`
  pins dist/serve/watch, and the README documents `trunk serve` /
  `trunk build --release`. The manual `wasm-bindgen` + `python -m
  http.server` steps are gone. `wasm-opt` runs in release with
  `--all-features` to accept rustc's bulk-memory output.
- `workspace` template gains a `targets/wasm` member: the same
  `app::build_ui` shared library now drives a browser build through
  the `web-canvas` backend, alongside desktop and ESP32-C3.

### Fixed

- Generated iOS and Android crates check on non-mobile hosts without importing mobile-only runtime entry points.
- The workspace defaults to the shared app and desktop target so a root-level `cargo check` does not combine incompatible platform dependencies.
- Generated desktop and shared UI source passes `cargo fmt --check`. CI checks formatting and host builds alongside target-specific builds.

### Removed

- The `sdl-only` generator path. Select SDL through `templates/desktop`.

## [0.1.0] — 2026-05-29

### Added

- `sdl-only` template: single-crate mirui application targeting the
  SDL2 desktop backend. Compiles on macOS / Linux with `libsdl2-dev`
  available; macOS Apple Silicon needs `LIBRARY_PATH=/opt/homebrew/lib`
  for the linker.
- `esp32c3` template: single-crate mirui application targeting
  ESP32-C3 with the esp-hal 1.1 stack. Ships a stub flush closure;
  copy `mirui-examples/examples/esp32c3-animation/src/board.rs` for
  real SPI / panel wiring. Cross-compiles to
  `riscv32imc-unknown-none-elf` out of the box.
- `workspace` template: Cargo workspace sharing a UI library across
  multiple target binaries, with starters for desktop (SDL) and
  ESP32-C3. The `targets/*` member glob picks new target crates up
  automatically — README documents the four-step recipe to add an
  ESP32-S3, RP2040, STM32, or any other MCU.
- `wasm` template: placeholder for an upcoming `web-canvas` Surface
  backend on mirui. Does not build yet; the README opens with the
  status banner.
- GitHub Actions CI: cargo-generate + cargo build matrix across all
  four templates (the wasm job verifies generation only — no build).
