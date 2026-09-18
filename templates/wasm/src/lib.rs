//! `{{project-name}}` on the `web-canvas` backend.

#![cfg(target_arch = "wasm32")]

use mirui::prelude::*;
use mirui::render::web_canvas::WebCanvasRendererFactory;
use mirui::surface::web_canvas::WebCanvasSurface;
use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;

#[path = "ui.rs"]
mod template_app;

#[wasm_bindgen(start)]
pub fn start() {
    console_error_panic_hook::set_once();

    let canvas = web_sys::window()
        .expect("window")
        .document()
        .expect("document")
        .get_element_by_id("mirui")
        .expect("canvas element with id=\"mirui\"")
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .expect("element is not <canvas>");

    let surface = WebCanvasSurface::new(canvas);
    let factory = WebCanvasRendererFactory::new();
    let mut app = App::with_factory(surface, factory);
    app.with_default_widgets().with_default_systems();

    let root = app.spawn_root().id();
    template_app::build_ui(&mut app.world, root);
    app.into_runner().start_animation_frame();
}
