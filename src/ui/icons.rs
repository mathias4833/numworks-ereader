use embedded_graphics::draw_target::DrawTarget;
use embedded_graphics::pixelcolor::Rgb565;
use embedded_graphics::prelude::*;
use embedded_graphics::primitives::Rectangle;
use embedded_graphics::{Drawable, Pixel};

const ICON_WIDTH: u32 = 13;
const ICON_HEIGHT: u32 = 12;

#[derive(Clone, Copy)]
pub enum Icon {
    Book,
    Folder,
    Recent,
    Settings,
}

pub struct IconView {
    icon: Icon,
    area: Rectangle,
    color: Rgb565,
}

impl IconView {
    pub const fn new(icon: Icon, area: Rectangle, color: Rgb565) -> Self {
        Self { icon, area, color }
    }
}

impl Drawable for IconView {
    type Color = Rgb565;
    type Output = ();

    fn draw<D>(&self, display: &mut D) -> Result<Self::Output, D::Error>
    where
        D: DrawTarget<Color = Self::Color>,
    {
        let bitmap = bitmap_for(self.icon);
        let offset = Point::new(
            self.area.top_left.x + self.area.size.width.saturating_sub(ICON_WIDTH) as i32 / 2,
            self.area.top_left.y + self.area.size.height.saturating_sub(ICON_HEIGHT) as i32 / 2,
        );

        for (y, row) in bitmap.iter().enumerate() {
            for x in 0..ICON_WIDTH {
                if row & bit_at(x) == 0 {
                    continue;
                }

                display.draw_iter(core::iter::once(Pixel(
                    offset + Point::new(x as i32, y as i32),
                    self.color,
                )))?;
            }
        }

        Ok(())
    }
}

const fn bit_at(x: u32) -> u16 {
    1 << (ICON_WIDTH - 1 - x)
}

fn bitmap_for(icon: Icon) -> &'static [u16; ICON_HEIGHT as usize] {
    match icon {
        Icon::Book => &BOOK,
        Icon::Folder => &FOLDER,
        Icon::Recent => &RECENT,
        Icon::Settings => &SETTINGS,
    }
}

const BOOK: [u16; ICON_HEIGHT as usize] = [
    0b0011111110000,
    0b0010000010000,
    0b0010100010000,
    0b0010100010000,
    0b0010100010000,
    0b0010100010000,
    0b0010100010000,
    0b0010100010000,
    0b0010100010000,
    0b0010100010000,
    0b0010000010000,
    0b0011111110000,
];

const FOLDER: [u16; ICON_HEIGHT as usize] = [
    0b0000000000000,
    0b0111100000000,
    0b0100011111100,
    0b1100000000100,
    0b1000000000100,
    0b1000000000100,
    0b1000000000100,
    0b1000000000100,
    0b1000000000100,
    0b1000000000100,
    0b1000000000100,
    0b1111111111100,
];

const RECENT: [u16; ICON_HEIGHT as usize] = [
    0b0011111111000,
    0b0010000001000,
    0b0010000001000,
    0b0010111101000,
    0b0010000001000,
    0b0010111101000,
    0b0010000001000,
    0b0010111101000,
    0b0010000001000,
    0b0010000001000,
    0b0010000001000,
    0b0011111111000,
];

const SETTINGS: [u16; ICON_HEIGHT as usize] = [
    0b0000000000000,
    0b0000000000000,
    0b1111111111110,
    0b0000100000000,
    0b0000100000000,
    0b1111111111110,
    0b0000000010000,
    0b0000000010000,
    0b1111111111110,
    0b0010000000000,
    0b0010000000000,
    0b0000000000000,
];
