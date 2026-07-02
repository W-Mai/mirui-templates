use mirui::prelude::*;
use mirui::surface::sdl::SdlSurface;
use mirui::ui::widgets::Text;

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

        Column (grow: 1.0, padding: Padding::all(16)) {
            View (
                bg_color: ColorToken::Primary,
                text_color: ColorToken::OnPrimary,
                height: 40,
                border_radius: 8,
                padding: Padding::all(10)
            ) {
                Text ("{{project-name}}")
            }
            View (bg_color: ColorToken::SurfaceVariant, grow: 1.0)
            View (height: 30, padding: Padding::all(6)) {
                Text ("Built with mirui")
            }
        }
    };

    app.run();
}
