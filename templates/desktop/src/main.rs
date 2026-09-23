use mirui::prelude::*;
{% if backend == "sdl" %}
use mirui::surface::sdl::SdlSurface;
{% else %}
use mirui::render::wgpu::WgpuRendererFactory;
use mirui::surface::wgpu_surface::WgpuSurface;
{% endif %}

#[path = "ui.rs"]
mod template_app;

fn main() {
{% if backend == "sdl" %}
    let backend = SdlSurface::new("{{project-name}}", 480, 320);
    let mut app = App::new(backend);
{% else %}
    let backend = WgpuSurface::new("{{project-name}}", 480, 320);
    let mut app = App::with_factory(backend, WgpuRendererFactory::new());
{% endif %}
    app.with_default_widgets().with_default_systems();

    let root = app.spawn_root().id();
    template_app::build_ui(&mut app.world, root);
    app.run();
}
