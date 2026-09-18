#![no_std]
#![no_main]

extern crate alloc;

use esp_alloc as _;

use mirui::prelude::*;
use mirui::render::texture::ColorFormat;
use mirui::surface::framebuf::FramebufSurface;

use app as template_app;

esp_bootloader_esp_idf::esp_app_desc!();

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    esp_println::println!("[PANIC] {}", info);
    loop {
        core::hint::spin_loop();
    }
}

const W: u16 = 160;
const H: u16 = 80;

#[esp_hal::main]
fn main() -> ! {
    esp_alloc::heap_allocator!(size: 72 * 1024);
    let _peripherals = esp_hal::init(esp_hal::Config::default());

    let backend = FramebufSurface::with_format(
        W,
        H,
        ColorFormat::RGB565Swapped,
        |_bytes: &[u8], _area: PhysicalRect| {
            // TODO: write `_bytes` to the LCD window described by `_area`.
        },
    );

    let mut app = App::new(backend);
    app.with_default_widgets().with_default_systems();

    let root = app.spawn_root().id();
    template_app::build_ui(&mut app.world, root);
    app.run();
    unreachable!();
}
