use mirui::prelude::*;
use mirui::ui::widgets::{Button, Text};
use winit::platform::android::activity::AndroidApp;

fn build_ui<B, F>(app: &mut App<B, F>)
where
    B: mirui::surface::Surface,
    F: mirui::render::factory::RendererFactory<B>,
{
    app.with_default_widgets().with_default_systems();
    let root = app.spawn_root().id();

    ui! {
        :(
            parent: root
            world: &mut app.world
        :)

        Column (
            grow: 1.0,
            padding: Padding::all(24),
            row_gap: 16,
            justify: JustifyContent::Center,
            align: AlignItems::Center
        ) {
            Text ("{{project-name}}", font_size: 30)
            Text ("mirui · {{backend}}", color: ColorToken::OnSurfaceVariant)
            Button ("READY", width: 160, height: 48)
        }
    };
}

#[unsafe(no_mangle)]
pub fn android_main(android_app: AndroidApp) {
{% if backend == "sw" %}    use mirui::surface::wgpu_upload::{SoftwareUploadConfig, software_mobile_host};

    software_mobile_host("{{project-name}}", SoftwareUploadConfig::default(), |surface| {
        let mut app = App::new(surface);
        build_ui(&mut app);
        app
    })
    .run_android(android_app);
{% else %}    use mirui::render::wgpu::WgpuRendererFactory;
    use mirui::surface::wgpu_surface::wgpu_mobile_host;

    wgpu_mobile_host("{{project-name}}", |surface| {
        let mut app = App::with_factory(surface, WgpuRendererFactory::new());
        build_ui(&mut app);
        app
    })
    .run_android(android_app);
{% endif %}}
