use mirui::prelude::*;
use mirui::surface::sdl::SdlSurface;

fn main() {
    let backend = SdlSurface::new("{{project-name}}", 480, 320);
    let mut app = App::new(backend);
    app.with_default_widgets().with_default_systems();

    let root = WidgetBuilder::new(&mut app.world)
        .bg_color(ColorToken::Surface)
        .layout(LayoutStyle {
            direction: FlexDirection::Column,
            width: Dimension::px(480),
            height: Dimension::px(320),
            ..Default::default()
        })
        .id();

    ui! {
        :(
            parent: root
            world: &mut app.world
        :)

        column (direction: FlexDirection::Column, grow: 1.0) {
            header (
                bg_color: ColorToken::Primary,
                text_color: ColorToken::OnPrimary,
                height: 40,
                text: "{{project-name}}",
                border_radius: 8
            ) {}
            content (bg_color: ColorToken::SurfaceVariant, grow: 1.0) {}
            footer (height: 30, text: "Built with mirui") {}
        }
    };

    app.set_root(root);
    app.run();
}
