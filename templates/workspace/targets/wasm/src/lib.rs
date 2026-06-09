//! `{{project-name}}` web target — `web-canvas` backend.

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
    let mut app_inst = App::with_factory(surface, factory);
    app_inst.with_default_widgets().with_default_systems();

    let root = WidgetBuilder::new(&mut app_inst.world)
        .bg_color(ColorToken::Surface)
        .layout(LayoutStyle {
            direction: FlexDirection::Column,
            width: Dimension::px(480),
            height: Dimension::px(320),
            ..Default::default()
        })
        .id();

    app::build_ui(&mut app_inst.world, root);

    app_inst.set_root(root);
    app_inst.into_runner().start_animation_frame();
}
