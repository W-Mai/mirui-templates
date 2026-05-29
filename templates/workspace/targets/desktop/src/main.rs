use mirui::prelude::*;
use mirui::surface::sdl::SdlSurface;

fn main() {
    let backend = SdlSurface::new("{{project-name}}", 480, 320);
    let mut app_inst = App::new(backend);
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
    app_inst.run();
}
