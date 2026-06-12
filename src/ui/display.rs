use crate::eadk;
use crate::eadk::display::{DISPLAY_HEIGHT, DISPLAY_WIDTH};
use core::convert::Infallible;
use embedded_graphics::draw_target::DrawTarget;
use embedded_graphics::geometry::{OriginDimensions, Size};
use embedded_graphics::pixelcolor::Rgb565;
use embedded_graphics::primitives::Rectangle;
use embedded_graphics::Pixel;

pub struct EadkDisplay;

impl EadkDisplay {
    pub const fn new() -> Self {
        Self
    }
}

impl OriginDimensions for EadkDisplay {
    fn size(&self) -> Size {
        Size::new(DISPLAY_WIDTH as u32, DISPLAY_HEIGHT as u32)
    }
}

impl DrawTarget for EadkDisplay {
    type Color = Rgb565;
    type Error = Infallible;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        for Pixel(point, color) in pixels {
            eadk::display::push_rect_uniform(Rectangle::new(point, Size::new(1, 1)), color);
        }

        Ok(())
    }

    fn fill_solid(&mut self, area: &Rectangle, color: Self::Color) -> Result<(), Self::Error> {
        eadk::display::push_rect_uniform(*area, color);
        Ok(())
    }
}
