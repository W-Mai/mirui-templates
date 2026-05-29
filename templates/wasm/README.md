# {{project-name}}

> **⚠ This template is a placeholder.** The mirui `web-canvas` Surface
> backend has not shipped yet — `cargo build` against this template
> will fail until it does. The template lives in the repo so its shape
> is committed and reviewable; once the backend lands, the same
> `cargo generate` command will produce a working project.
>
> Track progress in
> [mirui#1.0 surface platforms](https://github.com/W-Mai/mirui).

A future [mirui](https://github.com/W-Mai/mirui) application targeting
WebAssembly via a software-rendered HTML canvas backend.

## Build (will fail today)

```bash
cargo install wasm-pack
wasm-pack build --target web
python3 -m http.server   # serve index.html
```

`wasm-pack build` will fail to resolve `mirui`'s `web-canvas` feature
because that feature does not exist in any released version yet.

## What this template will do, once the backend ships

- `Cargo.toml` enables `mirui`'s `web-canvas` feature, which pulls in
  the `WebCanvasSurface` Surface implementation that draws into an
  HTML canvas via `web-sys`.
- `src/lib.rs`'s `#[wasm_bindgen(start)] fn run()` will be the
  browser-side entry. Today it just logs a console warning.
- `index.html` hosts a `<canvas id="mirui-canvas">` plus a wasm-bindgen
  module loader from `pkg/`.

The pattern matches the slint and bevy wasm setups: `wasm-pack build`
produces a `pkg/` next to `index.html`, served by any static HTTP
server.

## Use a different mirui template instead

Until the wasm backend ships, the working templates are:

- `sdl-only` — desktop SDL2 hello.
- `esp32c3` — ESP32-C3 embedded skeleton.
- `workspace` — Cargo workspace sharing UI code across desktop + ESP.

See the [mirui-templates README](https://github.com/W-Mai/mirui-templates).

## License

MIT — replace as needed.
