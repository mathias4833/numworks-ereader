#![no_std]
#![no_main]

mod eadk;

use eadk::{Color, Rect};

#[used]
#[unsafe(link_section = ".rodata.eadk_app_name")]
pub static EADK_APP_NAME: [u8; 10] = *b"HelloRust\0";

#[used]
#[unsafe(link_section = ".rodata.eadk_api_level")]
pub static EADK_APP_API_LEVEL: u32 = 0;

#[used]
#[unsafe(link_section = ".rodata.eadk_app_icon")]
pub static EADK_APP_ICON: [u8; 4250] = *include_bytes!("../target/icon.nwi");

#[unsafe(no_mangle)]
pub extern "C" fn main() -> ! {
    let red = Color { rgb565: 0xF800 };

    let rect = Rect {
        x: 40,
        y: 40,
        width: 240,
        height: 160,
    };

    eadk::display::push_rect_uniform(rect, red);

    loop {}
}
