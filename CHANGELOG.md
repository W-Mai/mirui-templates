# Changelog

All notable changes to mirui-templates are documented here. Versions
track the [mirui](https://github.com/W-Mai/mirui) release cycle: when
mirui ships a new minor, `cargo xtask templates-bump` inside the mirui
repo updates the per-template `cargo-generate.toml` defaults and lands
a matching commit here.

The format follows [Keep a Changelog](https://keepachangelog.com/en/1.1.0/).

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
