# {{project-name}}

A [mirui](https://github.com/W-Mai/mirui) application targeting
WebAssembly via the `web-canvas` Surface backend (Canvas 2D, no GPU).

## Build & run locally

```bash
# 1. wasm32 target + wasm-bindgen-cli (one-time)
rustup target add wasm32-unknown-unknown
cargo install wasm-bindgen-cli --version 0.2.122

# 2. compile
cargo build --release --target wasm32-unknown-unknown

# 3. wasm-bindgen post-process
wasm-bindgen \
  --target web \
  --out-dir pkg \
  target/wasm32-unknown-unknown/release/{{crate_name}}.wasm

# 4. serve
python3 -m http.server 8080
# → open http://localhost:8080
```

## Layout

- `src/lib.rs` — `#[wasm_bindgen(start)]` grabs `<canvas id="mirui">`,
  wraps it in `WebCanvasSurface`, builds the widget tree, and hands
  off to `Runner::start_animation_frame`.
- `index.html` — `<canvas>` plus a `<script type="module">` that
  imports the wasm-bindgen-generated JS glue from `pkg/`.

## Customising

- Swap the placeholder column for your own `ui!` tree.
- Resize the `<canvas>` via CSS in `index.html`; the Rust side picks
  up `client_width / client_height × devicePixelRatio` on startup.
- `web-canvas` is a Canvas 2D backend — for a GPU pipeline (WebGPU /
  WebGL2 fallback) wire mirui's `wgpu` feature instead through
  wasm-bindgen.

## Other mirui templates

- `sdl-only` — desktop SDL2 hello.
- `esp32c3` — ESP32-C3 embedded skeleton.
- `workspace` — Cargo workspace sharing UI code across desktop + ESP +
  wasm.

## License

MIT — replace as needed.
