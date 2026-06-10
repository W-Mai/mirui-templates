use mirui::prelude::*;
use mirui::surface::sdl::SdlSurface;

fn main() {
    let backend = SdlSurface::new("{{project-name}}", 480, 320);
    let mut app = App::new(backend);
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
                height: 40,
                text: "{{project-name}}",
                border_radius: 8
            )
            View (bg_color: ColorToken::SurfaceVariant, grow: 1.0)
            View (height: 30, text: "Built with mirui")
        }
    };

    app.run();
}
