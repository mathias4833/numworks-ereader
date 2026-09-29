use core::convert::Infallible;
use embedded_graphics::Pixel;
use embedded_graphics::pixelcolor::{Rgb565, RgbColor};
use embedded_graphics::prelude::{Dimensions, DrawTarget};
use embedded_graphics::primitives::Rectangle;

// Worse case, one BookCard (304x101 px), one MenuRow (304x41 px) and the gap between them (8 px)
// must be rendered. 304 * (101 + 8 + 41) = 45600
pub const BUFFER_PIXELS: usize = 46000;
static mut BUFFER: [Rgb565; BUFFER_PIXELS] = [Rgb565::BLACK; BUFFER_PIXELS];

pub struct RenderBuffer {
    area: Rectangle,
    pixels: &'static mut [Rgb565],
}

impl RenderBuffer {
    pub fn new() -> Self {
        // Only one RenderBuffer is created, so no other mutable reference to BUFFER exists.
        let pixels: &'static mut [Rgb565] = unsafe {
            core::slice::from_raw_parts_mut((&raw mut BUFFER).cast::<Rgb565>(), BUFFER_PIXELS)
        };

        Self {
            area: Rectangle::zero(),
            pixels,
        }
    }

    pub fn set_area(&mut self, area: Rectangle) {
        assert!(self.can_hold(area));
        self.area = area;
    }

    pub fn pixels(&self) -> &[Rgb565] {
        &self.pixels[..Self::pixel_count(self.area)]
    }

    pub fn pixel_count(area: Rectangle) -> usize {
        area.size.width as usize * area.size.height as usize
    }

    pub fn can_hold(&self, area: Rectangle) -> bool {
        Self::pixel_count(area) <= self.pixels.len()
    }
}

impl Dimensions for RenderBuffer {
    fn bounding_box(&self) -> Rectangle {
        self.area
    }
}

impl DrawTarget for RenderBuffer {
    type Color = Rgb565;
    type Error = Infallible;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Rgb565>>,
    {
        let stride = self.area.size.width as usize;

        for Pixel(point, color) in pixels {
            if !self.area.contains(point) {
                continue;
            }

            let x = (point.x - self.area.top_left.x) as usize;
            let y = (point.y - self.area.top_left.y) as usize;

            self.pixels[y * stride + x] = color;
        }

        Ok(())
    }

    fn fill_solid(&mut self, area: &Rectangle, color: Self::Color) -> Result<(), Self::Error> {
        let area = area.intersection(&self.area);
        if area.is_zero_sized() {
            return Ok(());
        }

        let stride = self.area.size.width as usize;
        let x = (area.top_left.x - self.area.top_left.x) as usize;
        let y = (area.top_left.y - self.area.top_left.y) as usize;
        let width = area.size.width as usize;
        let height = area.size.height as usize;

        self.pixels
            .chunks_mut(stride)
            .skip(y)
            .take(height)
            .for_each(|row| row[x..x + width].fill(color));

        Ok(())
    }
}
