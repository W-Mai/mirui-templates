#![no_std]
#![no_main]

extern crate alloc;

use esp_alloc as _;

use mirui::draw::texture::ColorFormat;
use mirui::prelude::*;
use mirui::surface::framebuf::FramebufSurface;
use mirui::types::Rect;

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

    // BSP wiring: replace the stub closure below with a real SPI
    // driver that pushes `bytes` to your panel for the rectangle
    // described by `area`. See
    // https://github.com/W-Mai/mirui-examples/tree/main/examples/esp32c3-animation
    // for a complete ST7735S setup (board.rs covers SPI + DMA + reset).
    //
    // `RGB565Swapped` is the byte order most ST7735/ST7789 panels expect
    // when the host MCU is little-endian; swap to `RGB565` if your panel
    // takes the bytes the other way.
    let backend = FramebufSurface::with_format(
        W, H,
        ColorFormat::RGB565Swapped,
        |_bytes: &[u8], _area: &Rect| {
            // TODO: write `_bytes` to the LCD window described by `_area`
        },
    );

    let mut app = App::new(backend);
    app.with_default_widgets().with_default_systems();

    let root = WidgetBuilder::new(&mut app.world)
        .bg_color(ColorToken::Surface)
        .id();

    ui! {
        :(
            parent: root
            world: &mut app.world
        :)

        hello (
            bg_color: ColorToken::Primary,
            text_color: ColorToken::OnPrimary,
            text: "{{project-name}}"
        ) {}
    };

    app.set_root(root);
    app.run();
    unreachable!();
}
