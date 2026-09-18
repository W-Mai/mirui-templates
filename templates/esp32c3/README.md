# {{project-name}}

A [mirui](https://github.com/W-Mai/mirui) application targeting ESP32-C3 with an SPI display on the `esp-hal` 1.1 stack.

## Status

This template provides the mirui application skeleton with a stub framebuffer flush closure. Copy the board integration from [`mirui-examples`](https://github.com/W-Mai/mirui-examples/tree/main/examples/esp32c3-animation), adjust its SPI, panel, and DMA wiring, and call it from `main`.

`cargo build --release` succeeds with the stub and verifies the mirui prelude, `App`, `ui!`, and `FramebufSurface` callback boundary.

## Build

```bash
cargo build --release
```

The output binary lands at `target/riscv32imc-unknown-none-elf/release/{{project-name}}`.

## Flash

```bash
cargo install espflash
espflash flash --monitor target/riscv32imc-unknown-none-elf/release/{{project-name}}
```

With the runner configured in `.cargo/config.toml`, `cargo run --release` builds and flashes the firmware.

## Add a real SPI panel

1. Copy `mirui-examples/examples/esp32c3-animation/src/board.rs` into this project's `src/`.
2. Adjust the pin assignments in `board.rs` to match the board wiring.
3. Replace the stub closure inside `FramebufSurface::with_format` with the panel flush callback.
4. Match `ColorFormat` to the panel byte order; ST7735 and ST7789 panels driven from a little-endian MCU commonly use `RGB565Swapped`.

## Pinned dependency versions

- `mirui` requires `{{mirui-version}}` and resolves from the repository revision recorded by the template
- `esp-hal = "1.1"`
- `esp-alloc = "0.7"`
- `esp-bootloader-esp-idf = "0.5"`
- `esp-println = "0.14"`
- `critical-section = "1.2"`

These versions track `mirui-examples/examples/esp32c3-animation` and should be updated together.

## Optional features

| Feature | Effect |
|---------|--------|
| `quad-aa` | Enables antialiasing for transformed quads at additional CPU cost |
| `perf` | Enables mirui frame timing resources |

Both features are disabled by default. Build with `cargo build --release --features quad-aa` when edge quality is more important than the smallest firmware path.

## License

MIT — replace with your own as needed.
