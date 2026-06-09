# {{project-name}}

A [mirui](https://github.com/W-Mai/mirui) application targeting
WebAssembly via the `web-canvas` Surface backend (Canvas 2D, no GPU).

## Prerequisites

```bash
rustup target add wasm32-unknown-unknown
cargo install --locked trunk
```

[trunk](https://trunkrs.dev) bundles the wasm build, runs
`wasm-bindgen` and `wasm-opt`, serves the page, and live-reloads on
file changes.

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

`trunk serve` / bare `trunk build` use the dev profile — fast to
rebuild, large wasm (debug symbols, no optimization), never shipped.
`--release` applies `opt-level = "z"` + `lto` + `strip` and drops the
wasm to a fraction of the dev size. `wasm-opt` runs only in release
and is wired with `--all-features` in `index.html` so it accepts the
bulk-memory ops rustc emits for wasm32.

## Layout

- `src/lib.rs` — `#[wasm_bindgen(start)]` grabs `<canvas id="mirui">`,
  wraps it in `WebCanvasSurface`, builds the widget tree, and hands
  off to `Runner::start_animation_frame`. trunk calls the `start`
  export automatically.
- `index.html` — `<canvas>` plus `<link data-trunk rel="rust">` that
  tells trunk to compile this crate to wasm and inject the loader.

## Customising

- Swap the placeholder column for your own `ui!` tree.
- Resize the `<canvas>` via CSS in `index.html`; the Rust side picks
  up `client_width / client_height × devicePixelRatio` on startup.
- `web-canvas` is a Canvas 2D backend — for a GPU pipeline (WebGPU /
  WebGL2 fallback) wire mirui's `wgpu` feature instead.

## Other mirui templates

- `sdl-only` — desktop SDL2 hello.
- `esp32c3` — ESP32-C3 embedded skeleton.
- `workspace` — Cargo workspace sharing UI code across desktop + ESP +
  wasm.

## License

MIT — replace as needed.
