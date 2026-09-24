use embedded_graphics::geometry::Size;
use embedded_graphics::pixelcolor::Rgb565;
use embedded_graphics::prelude::RgbColor;
use embedded_graphics::primitives::{PrimitiveStyle, PrimitiveStyleBuilder};
use u8g2_fonts::{U8g2TextStyle, fonts};

pub const BACKGROUND: Rgb565 = Rgb565::WHITE;
pub const FOREGROUND: Rgb565 = Rgb565::BLACK;

pub const SURFACE: Rgb565 = Rgb565::new(30, 60, 30);
pub const SURFACE_SELECTED: Rgb565 = Rgb565::new(26, 53, 26);

pub const BORDER: Rgb565 = Rgb565::new(28, 57, 28);
pub const BORDER_SELECTED: Rgb565 = Rgb565::new(15, 30, 15);

pub const CORNER_RADIUS: Size = Size::new(8, 8);

pub fn surface(selected: bool) -> PrimitiveStyle<Rgb565> {
    let (background, border) = if selected {
        (SURFACE_SELECTED, BORDER_SELECTED)
    } else {
        (SURFACE, BORDER)
    };

    PrimitiveStyleBuilder::new()
        .fill_color(background)
        .stroke_color(border)
        .stroke_width(1)
        .build()
}

pub fn text(color: Rgb565) -> U8g2TextStyle<Rgb565> {
    U8g2TextStyle::new(fonts::u8g2_font_helvR10_tr, color)
}

pub fn monospace_text(color: Rgb565) -> U8g2TextStyle<Rgb565> {
    U8g2TextStyle::new(fonts::u8g2_font_8x13_mr, color)
}
