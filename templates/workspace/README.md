# {{project-name}}

A Cargo workspace that drives the same [mirui](https://github.com/W-Mai/mirui) UI on desktop, WebAssembly, ESP32-C3, Android, and iOS.

The shared application starts with an interactive mode card and live slider. Mobile roots respect the current safe area automatically, while desktop, browser, and embedded surfaces report zero insets.

## Layout

```
{{project-name}}/
├── Cargo.toml             # [workspace] members = ["app", "targets/*"]
├── app/                   # shared UI library, std/no_std dual
│   ├── Cargo.toml
│   └── src/lib.rs         # pub fn build_ui(world, parent)
└── targets/
    ├── desktop/           # WGPU or SDL2 binary
    │   ├── Cargo.toml
    │   └── src/main.rs
    ├── esp32c3/           # ESP32-C3 binary, framebuffer + esp-hal stack
    │   ├── Cargo.toml
    │   ├── .cargo/config.toml
    │   ├── rust-toolchain.toml
    │   └── src/main.rs
    ├── wasm/              # browser build, web-canvas backend via trunk
    │   ├── Cargo.toml
    │   ├── Trunk.toml
    │   ├── index.html
    │   └── src/lib.rs
    ├── android/           # Android NativeActivity host
    │   ├── Cargo.toml
    │   └── src/lib.rs
    └── ios/               # iOS UIKit host and Xcode project
        ├── Cargo.toml
        ├── ios/
        └── src/lib.rs
```

The `targets/*` glob in the workspace manifest picks up new target directories automatically. A plain `cargo check` or `cargo build` at the workspace root checks only the shared app and desktop target. Build ESP32-C3, browser, Android, and iOS targets with their platform-specific commands below; `--workspace` cannot combine their target-specific dependencies in a single host build.

## Build and run

Desktop:

```bash
cargo run -p desktop
```

The generated desktop target uses the selected `desktop-backend` (`wgpu` by default). Selecting `sdl` builds and statically links SDL2; install a C compiler and CMake, but no separate SDL2 development package or library path is needed. The executable still depends on its operating system's libraries.

ESP32-C3:

```bash
cd targets/esp32c3
cargo build --release
cargo install espflash
espflash flash --monitor ../../target/riscv32imc-unknown-none-elf/release/target-esp32c3
```

The ESP target defaults to the smallest mirui configuration. Add `--features quad-aa` for transformed-edge antialiasing or `--features perf` for frame timing resources.

The `cd` matters because Cargo reads `.cargo/config.toml` from the current directory's tree. The `riscv32imc-unknown-none-elf` target lives in `targets/esp32c3/.cargo/config.toml`; running `cargo build -p target-esp32c3` from the workspace root selects the host architecture instead.

Web (browser):

```bash
rustup target add wasm32-unknown-unknown
cargo install --locked trunk
cd targets/wasm
trunk serve          # dev server at http://127.0.0.1:8080 with live reload
trunk build --release # production bundle in dist/
```

[trunk](https://trunkrs.dev) handles the WebAssembly build, `wasm-bindgen`, `wasm-opt`, and serving. `trunk serve` uses the development profile; `--release` applies the workspace release profile. `wasm-opt` receives `--all-features` in `index.html` so it accepts the bulk-memory operations emitted by rustc.

Android:

```bash
rustup target add aarch64-linux-android
cargo install cargo-apk
cd targets/android
cargo apk run --release
```

The generated package uses the selected `android-backend`: `wgpu` renders directly and `sw` rasterizes in software before uploading dirty regions through WGPU.

iOS:

```bash
rustup target add aarch64-apple-ios aarch64-apple-ios-sim
open targets/ios/ios/{{project-name}}.xcodeproj
```

Select an iOS Simulator or a signing team and physical device in Xcode. The generated target uses the selected `ios-backend`.

The `target-` package prefix avoids a collision with the `esp32c3` peripheral-access crate used by esp-hal.

The ESP target contains a stub framebuffer flush closure. Copy `board.rs` from [`mirui-examples`](https://github.com/W-Mai/mirui-examples/tree/main/examples/esp32c3-animation), adjust the board wiring, and call it from `main` to drive the display.

## Add a new target

ESP32-C3 is the cross-built embedded target included in this template. ESP32-S3, RP2040, STM32, and other MCUs are extension points rather than verified boards in this repository. To add one, replicate the existing target directory:

1. Copy a starting point: `cp -r targets/esp32c3 targets/<name>`.
2. Update the new crate's `Cargo.toml` package name and board-support dependencies.
3. Update `.cargo/config.toml` and `rust-toolchain.toml` for the target triple and linker script.
4. Adjust `src/main.rs` for the target clocks, SPI peripheral, and panel while retaining the mirui application wiring.

`cargo build -p <name>` picks up the new crate without an edit to the workspace manifest.

## Sharing UI code

The `app` library supports `std` and `no_std`, selected through its `std` feature:

- `targets/desktop` enables `app/std` for either desktop backend.
- `targets/esp32c3` keeps `app` at `default-features = false`.
- `targets/wasm` enables `app/std` (the `web-canvas` surface needs `std`).
- `targets/android` and `targets/ios` enable `app/std`.

Add shared mirui UI to `app/src/lib.rs`. Gate any `std`-only code with `#[cfg(feature = "std")]` so the embedded target remains buildable.

## License

MIT — replace as needed.
