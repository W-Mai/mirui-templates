# {{project-name}}

A [mirui](https://github.com/W-Mai/mirui) application targeting
ESP32-C3 with an SPI display, built on the `esp-hal` 1.1 stack.

## Status

This template ships only the mirui-side skeleton — the SPI / panel /
DMA wiring is intentionally a stub closure in `src/main.rs`. To get
pixels on screen, copy a working board file from
[`mirui-examples`](https://github.com/W-Mai/mirui-examples/tree/main/examples/esp32c3-animation)
and call it from `main`.

`cargo build --release` will succeed even with the stub — what's
verified is the framework wiring (mirui prelude, `App`, `ui!` macro,
`FramebufSurface` flush callback shape).

## Build

```bash
cargo build --release
```

The output binary lands at
`target/riscv32imc-unknown-none-elf/release/{{project-name}}`.

## Flash

```bash
cargo install espflash
espflash flash --monitor target/riscv32imc-unknown-none-elf/release/{{project-name}}
```

Or, with the runner already configured in `.cargo/config.toml`,
just `cargo run --release`.

## Add a real SPI panel

1. Copy `mirui-examples/examples/esp32c3-animation/src/board.rs` into
   this project's `src/`.
2. Adjust pin assignments at the top of `board.rs` to match your
   wiring.
3. In `main`, replace the stub closure inside `FramebufSurface::with_format`
   with the real flush callback that drives your panel.
4. Match `ColorFormat` to your panel's byte order (most ST7735/ST7789
   panels driven from a little-endian MCU want `RGB565Swapped`).

## Pinned dependency versions

- `mirui = "{{mirui-version}}"`
- `esp-hal = "1.1"`
- `esp-alloc = "0.7"`
- `esp-bootloader-esp-idf = "0.5"`
- `esp-println = "0.14"`
- `critical-section = "1.2"`

These track `mirui-examples/examples/esp32c3-animation`. Bump them
together when esp-hal cuts a new minor.

## License

MIT — replace with your own as needed.
