// Placeholder entry point for the wasm-bindgen build. This template
// will not build until mirui's `web-canvas` Surface backend ships —
// see README for the timeline.
//
// Once the backend is available, replace this file with the real
// `App::new(WebCanvasSurface::from_canvas_id("mirui-canvas"))` setup
// plus a `WasmRunner` driven by `requestAnimationFrame`.

use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn run() {
    web_sys::console::log_1(
        &"{{project-name}}: web-canvas backend not yet shipped — see README".into(),
    );
}
