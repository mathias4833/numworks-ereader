use crate::geometry::{Point, Rect};
use core::ffi::CStr;

#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct Color {
    pub rgb565: u16,
}

impl Color {
    pub const BLACK: Self = Self { rgb565: 0x0000 };
    pub const WHITE: Self = Self { rgb565: 0xffff };
    pub const RED: Self = Self { rgb565: 0xf800 };
    pub const GRAY: Self = Self { rgb565: 0x8410 };
    pub const GREEN: Self = Self { rgb565: 0x05e0 };
    pub const ORANGE: Self = Self { rgb565: 0xfd20 };
}

pub const DISPLAY_WIDTH: u16 = 320;
pub const DISPLAY_HEIGHT: u16 = 240;

pub fn push_rect_uniform(rect: Rect, color: Color) {
    unsafe {
        eadk_display_push_rect_uniform(rect, color);
    }
}

pub fn draw_string(
    text: &CStr,
    point: Point,
    large_font: bool,
    text_color: Color,
    background_color: Color,
) {
    unsafe {
        eadk_display_draw_string(
            text.as_ptr() as *const u8,
            point,
            large_font,
            text_color,
            background_color,
        );
    }
}

unsafe extern "C" {
    fn eadk_display_push_rect_uniform(rect: Rect, color: Color);
    fn eadk_display_draw_string(
        text: *const u8,
        point: Point,
        large_font: bool,
        text_color: Color,
        background_color: Color,
    );
}
