use mirui::prelude::*;
use mirui::surface::sdl::SdlSurface;

fn main() {
    let backend = SdlSurface::new("{{project-name}}", 480, 320);
    let mut app_inst = App::new(backend);
    app_inst.with_default_widgets().with_default_systems();

    let root = app_inst.spawn_root().id();

    app::build_ui(&mut app_inst.world, root);

    app_inst.run();
}
