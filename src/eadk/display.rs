use embedded_graphics::geometry::{Point, Size};
use embedded_graphics::pixelcolor::raw::RawU16;
use embedded_graphics::pixelcolor::Rgb565;
use embedded_graphics::prelude::RawData;
use embedded_graphics::primitives::Rectangle;

pub const DISPLAY_WIDTH: u16 = 320;
pub const DISPLAY_HEIGHT: u16 = 240;

#[repr(transparent)]
#[derive(Clone, Copy)]
struct EadkColor {
    rgb565: u16,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct EadkRect {
    x: u16,
    y: u16,
    width: u16,
    height: u16,
}

impl From<Rgb565> for EadkColor {
    fn from(color: Rgb565) -> Self {
        Self {
            rgb565: RawU16::from(color).into_inner(),
        }
    }
}

impl TryFrom<Rectangle> for EadkRect {
    type Error = ();

    fn try_from(rect: Rectangle) -> Result<Self, Self::Error> {
        let Point { x, y } = rect.top_left;
        let Size { width, height } = rect.size;

        if x < 0 || y < 0 || width == 0 || height == 0 {
            return Err(());
        }

        let right = x as u32 + width;
        let bottom = y as u32 + height;

        if right > DISPLAY_WIDTH as u32 || bottom > DISPLAY_HEIGHT as u32 {
            return Err(());
        }

        Ok(Self {
            x: x as u16,
            y: y as u16,
            width: width as u16,
            height: height as u16,
        })
    }
}

pub fn push_rect_uniform(rect: Rectangle, color: Rgb565) {
    if let Ok(rect) = EadkRect::try_from(rect) {
        unsafe {
            eadk_display_push_rect_uniform(rect, color.into());
        }
    }
}

// TODO
// pub fn push_rect(rect: Rectangle, pixels: &[Rgb565]) {
//     unsafe {
//         eadk_display_push_rect(rect, pixels.as_ptr());
//     }
// }

unsafe extern "C" {
    fn eadk_display_push_rect_uniform(rect: EadkRect, color: EadkColor);
    fn eadk_display_push_rect(rect: EadkRect, pixels: *const EadkColor);
}
