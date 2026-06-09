# {{project-name}}

A Cargo workspace that drives the same [mirui](https://github.com/W-Mai/mirui)
UI from multiple targets — a desktop binary against SDL2, an
ESP32-C3 binary against esp-hal, and a browser build against the
`web-canvas` backend — without duplicating the UI code.

## Layout

```
{{project-name}}/
├── Cargo.toml             # [workspace] members = ["app", "targets/*"]
├── app/                   # shared UI library, std/no_std dual
│   ├── Cargo.toml
│   └── src/lib.rs         # pub fn build_ui(world, parent)
└── targets/
    ├── desktop/           # SDL2 binary
    │   ├── Cargo.toml
    │   └── src/main.rs
    ├── esp32c3/           # ESP32-C3 binary, framebuffer + esp-hal stack
    │   ├── Cargo.toml
    │   ├── .cargo/config.toml
    │   ├── rust-toolchain.toml
    │   └── src/main.rs
    └── wasm/              # browser build, web-canvas backend via trunk
        ├── Cargo.toml
        ├── Trunk.toml
        ├── index.html
        └── src/lib.rs
```

The `targets/*` glob in the workspace manifest picks up new
sub-directories automatically — the next section explains how to add
one.

## Build and run

Desktop:

```bash
cargo run -p desktop
```

The first build pulls SDL2 from your system. On macOS Apple Silicon,
SDL2 from Homebrew installs at `/opt/homebrew/lib/`, which the linker
does not search by default — add it for this shell:

```bash
export LIBRARY_PATH="/opt/homebrew/lib:$LIBRARY_PATH"
```

ESP32-C3:

```bash
cd targets/esp32c3
cargo build --release
cargo install espflash
espflash flash --monitor ../../target/riscv32imc-unknown-none-elf/release/target-esp32c3
```

The `cd` matters: cargo only reads `.cargo/config.toml` from the
current directory's tree, and the `riscv32imc-unknown-none-elf` build
target lives in `targets/esp32c3/.cargo/config.toml`. Running
`cargo build -p target-esp32c3` from the workspace root falls back to
the host architecture and fails to compile `portable-atomic`.

Web (browser):

```bash
rustup target add wasm32-unknown-unknown
cargo install --locked trunk
cd targets/wasm
trunk serve          # dev server at http://127.0.0.1:8080 with live reload
trunk build --release # production bundle in dist/
```

[trunk](https://trunkrs.dev) handles the wasm build, `wasm-bindgen`,
`wasm-opt`, and serving. `trunk serve` uses the dev profile (fast
rebuild, large wasm); `--release` applies the workspace
`[profile.release]` and shrinks the output. `wasm-opt` runs only in
release with `--all-features` (wired in `index.html`) so it accepts
the bulk-memory ops rustc emits for wasm32.

(The `target-` prefix on the package name avoids a collision with the
`esp32c3` peripheral-access crate that esp-hal pulls in transitively.
The `desktop` crate doesn't collide with anything, so it stays
unprefixed. Pick whatever convention you like for new targets — see
below.)

The ESP target ships only a mirui-side skeleton — the SPI / panel /
DMA wiring is a stub closure in `targets/esp32c3/src/main.rs`. Copy a
working `board.rs` from
[`mirui-examples`](https://github.com/W-Mai/mirui-examples/tree/main/examples/esp32c3-animation)
and call it from `main` to drive a real LCD.

## Add a new target

To add ESP32-S3, RP2040, STM32, or any other MCU, replicate one of the
existing target directories:

1. Copy a starting point: `cp -r targets/esp32c3 targets/<name>`.
2. Update the new crate's `Cargo.toml` `[package].name` and BSP
   dependencies (clocks, peripherals, etc.).
3. Update `.cargo/config.toml` and `rust-toolchain.toml` for the new
   target triple and linker script.
4. Adjust `src/main.rs` to talk to the new chip's clocks, SPI, and
   panel. The mirui-side wiring (`App::new`, `app::build_ui`, `run`)
   stays the same.

`cargo build -p <name>` picks the new crate up automatically — no
edit to the workspace manifest needed.

## Sharing UI code

The `app` library is `std` / `no_std` dual, gated by the `std`
feature. Each target enables `std` exactly when its mirui surface
needs it:

- `targets/desktop` enables `app/std` (SDL backend pulls in `std`).
- `targets/esp32c3` keeps `app` at `default-features = false`.
- `targets/wasm` enables `app/std` (the `web-canvas` surface needs `std`).

Add new mirui usage inside `app/src/lib.rs` and both targets pick it
up unchanged. If you reach for an `std::` API in `app`, gate it with
`#[cfg(feature = "std")]` so the embedded build keeps compiling.

## License

MIT — replace as needed.
