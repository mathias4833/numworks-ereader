#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct Color {
    pub rgb565: u16,
}

impl Color {
    pub const BLACK: Self = Self { rgb565: 0x0000 };
    pub const WHITE: Self = Self { rgb565: 0xffff };
    pub const RED: Self = Self { rgb565: 0xf800 };
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Rect {
    pub x: u16,
    pub y: u16,
    pub width: u16,
    pub height: u16,
}

impl Rect {
    pub const fn new(x: u16, y: u16, width: u16, height: u16) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }
}

pub mod display {
    use super::{Color, Rect};

    pub const WIDTH: u16 = 320;
    pub const HEIGHT: u16 = 240;

    pub fn fill_screen(color: Color) {
        push_rect_uniform(Rect::new(0, 0, WIDTH, HEIGHT), color);
    }

    pub fn push_rect_uniform(rect: Rect, color: Color) {
        unsafe {
            eadk_display_push_rect_uniform(rect, color);
        }
    }

    unsafe extern "C" {
        fn eadk_display_push_rect_uniform(rect: Rect, color: Color);
    }
}

pub mod event {
    pub fn get(timeout: &mut i32) -> u8 {
        unsafe { eadk_event_get(timeout as *mut i32) }
    }

    unsafe extern "C" {
        fn eadk_event_get(timeout: *mut i32) -> u8;
    }
}
