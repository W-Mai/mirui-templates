# {{project-name}}

A [mirui](https://github.com/W-Mai/mirui) application targeting WebAssembly through the `web-canvas` Surface backend.

## Prerequisites

```bash
rustup target add wasm32-unknown-unknown
cargo install --locked trunk
```

[trunk](https://trunkrs.dev) bundles the WebAssembly build, runs `wasm-bindgen` and `wasm-opt`, serves the page, and reloads after source changes.

## Develop

```bash
trunk serve
# → open http://127.0.0.1:8080
```

Rebuilds and reloads the browser on every edit.

## Build for release

```bash
trunk build --release
# output in dist/
```

`trunk serve` and bare `trunk build` use the development profile. `--release` applies `opt-level = "z"`, LTO, and symbol stripping. `wasm-opt` runs only in release and receives `--all-features` in `index.html` so it accepts the bulk-memory operations emitted by rustc.

## Layout

- `src/lib.rs` — `#[wasm_bindgen(start)]` resolves `<canvas id="mirui">`, wraps it in `WebCanvasSurface`, builds the widget tree, and starts the animation-frame runner.
- `index.html` — `<canvas>` plus `<link data-trunk rel="rust">` for compiling the crate and injecting its loader.

## Customising

- Swap the placeholder column for your own `ui!` tree.
- Resize the `<canvas>` through CSS in `index.html`; the Rust side reads its client dimensions and device pixel ratio.
- Browser rendering uses the `web-canvas` backend; native WGPU targets use the Android, iOS, or desktop integration.

## Other mirui templates

- `desktop` — desktop WGPU or SDL2 application.
- `esp32c3` — ESP32-C3 embedded skeleton.
- `workspace` — Cargo workspace sharing UI code across desktop, ESP32-C3, and WebAssembly.

## License

MIT — replace as needed.
