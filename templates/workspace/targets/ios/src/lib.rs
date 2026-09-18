use mirui::prelude::*;

use app as template_app;

#[unsafe(no_mangle)]
pub extern "C" fn mirui_start() -> ! {
{% if ios-backend == "sw" %}    use mirui::surface::wgpu_upload::{SoftwareUploadConfig, software_mobile_host};

    software_mobile_host("{{project-name}}", SoftwareUploadConfig::default(), |surface| {
        let mut app = App::new(surface);
        app.with_default_widgets().with_default_systems();
        let root = app.spawn_root().id();
        template_app::build_ui(&mut app.world, root);
        app
    })
    .run_ios()
{% else %}    use mirui::render::wgpu::WgpuRendererFactory;
    use mirui::surface::wgpu_surface::wgpu_mobile_host;

    wgpu_mobile_host("{{project-name}}", |surface| {
        let mut app = App::with_factory(surface, WgpuRendererFactory::new());
        app.with_default_widgets().with_default_systems();
        let root = app.spawn_root().id();
        template_app::build_ui(&mut app.world, root);
        app
    })
    .run_ios()
{% endif %}}
