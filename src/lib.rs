#![cfg_attr(target_os = "none", no_std)]

pub mod eadk;

#[cfg(target_os = "none")]
mod panic;

use eadk::{Color, Rect};

#[used]
#[unsafe(no_mangle)]
#[unsafe(link_section = ".rodata.eadk_app_name")]
pub static EADK_APP_NAME: [u8; 10] = *b"HelloRust\0";
#[used]
#[unsafe(no_mangle)]
#[unsafe(link_section = ".rodata.eadk_api_level")]
pub static EADK_API_LEVEL: u32 = 0;
#[used]
#[unsafe(no_mangle)]
#[unsafe(link_section = ".rodata.eadk_app_icon")]
pub static EADK_APP_ICON: [u8; 4250] = *include_bytes!("../target/icon.nwi");

pub fn app_main() -> ! {
    eadk::display::push_rect_uniform(
        Rect {
            x: 0,
            y: 0,
            width: 320,
            height: 240,
        },
        Color { rgb565: 0xffff },
    );

    eadk::display::push_rect_uniform(
        Rect {
            x: 40,
            y: 40,
            width: 240,
            height: 160,
        },
        Color { rgb565: 0xf800 },
    );

    loop {
        let mut timeout = 20;
        eadk::event::get(&mut timeout);
    }
}

// For the emulator
#[cfg(not(target_os = "none"))]
#[unsafe(no_mangle)]
pub extern "C" fn main(_argc: i32, _argv: *const *const u8) -> i32 {
    app_main();
}
