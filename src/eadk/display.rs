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

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Point {
    pub x: u16,
    pub y: u16,
}

impl Point {
    pub const fn new(x: u16, y: u16) -> Self {
        Self { x, y }
    }
}

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
