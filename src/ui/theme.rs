use embedded_graphics::mono_font::MonoTextStyle;
use embedded_graphics::mono_font::ascii::{FONT_7X14, FONT_9X18};
use embedded_graphics::pixelcolor::Rgb565;
use embedded_graphics::prelude::RgbColor;

pub const BACKGROUND: Rgb565 = Rgb565::WHITE;
pub const FOREGROUND: Rgb565 = Rgb565::BLACK;

pub const SELECTED_BACKGROUND: Rgb565 = FOREGROUND;
pub const SELECTED_FOREGROUND: Rgb565 = BACKGROUND;

pub const SURFACE: Rgb565 = Rgb565::new(30, 60, 30);
pub const SURFACE_SELECTED: Rgb565 = Rgb565::new(26, 53, 26);

pub const BORDER: Rgb565 = Rgb565::new(28, 57, 28);
pub const BORDER_SELECTED: Rgb565 = Rgb565::new(15, 30, 15);

pub const DANGER: Rgb565 = Rgb565::RED;
pub const WARNING: Rgb565 = Rgb565::YELLOW;
pub const SUCCESS: Rgb565 = Rgb565::GREEN;

pub fn text(color: Rgb565) -> MonoTextStyle<'static, Rgb565> {
    MonoTextStyle::new(&FONT_7X14, color)
}

pub fn title(color: Rgb565) -> MonoTextStyle<'static, Rgb565> {
    MonoTextStyle::new(&FONT_9X18, color)
}
