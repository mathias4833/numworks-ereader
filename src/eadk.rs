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

pub mod event {
    pub fn get(timeout: &mut i32) -> u8 {
        unsafe { eadk_event_get(timeout as *mut i32) }
    }

    unsafe extern "C" {
        fn eadk_event_get(timeout: *mut i32) -> u8;
    }
}
