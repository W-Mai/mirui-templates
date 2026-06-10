//! `{{project-name}}` — mirui app on the `web-canvas` backend.

#![cfg(target_arch = "wasm32")]

use mirui::draw::web_canvas::WebCanvasRendererFactory;
use mirui::prelude::*;
use mirui::surface::web_canvas::WebCanvasSurface;
use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;

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

    ui! {
        :(
            parent: root
            world: &mut app.world
        :)

        Column (grow: 1.0) {
            View (
                bg_color: ColorToken::Primary,
                text_color: ColorToken::OnPrimary,
                height: 48,
                text: "{{project-name}}",
                border_radius: 8
            )
            View (bg_color: ColorToken::SurfaceVariant, grow: 1.0)
            View (height: 32, text: "mirui · web-canvas")
        }
    };

    app.into_runner().start_animation_frame();
}
