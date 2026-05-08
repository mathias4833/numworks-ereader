#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct Color {
    pub rgb565: u16,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Rect {
    pub x: u16,
    pub y: u16,
    pub width: u16,
    pub height: u16,
}

pub mod display {
    use super::{Color, Rect};

    pub fn push_rect_uniform(rect: Rect, color: Color) {
        unsafe {
            eadk_display_push_rect_uniform(rect, color);
        }
    }

    unsafe extern "C" {
        fn eadk_display_push_rect_uniform(rect: Rect, color: Color);
    }
}

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_panic: &PanicInfo<'_>) -> ! {
    loop {}
}
