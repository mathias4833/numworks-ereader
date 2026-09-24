mod frame;
mod insets;
mod stack;

use crate::eadk::display::{DISPLAY_HEIGHT, DISPLAY_WIDTH};
use embedded_graphics::geometry::{Point, Size};
use embedded_graphics::primitives::Rectangle;
pub use frame::Frame;
pub use insets::Insets;
pub use stack::Stack;

pub fn centered_line_y(area: Rectangle, line_height: u32) -> i32 {
    area.top_left.y + (area.size.height - line_height).div_ceil(2) as i32
}

pub const SCREEN: Rectangle = Rectangle::new(
    Point::new(0, 0),
    Size::new(DISPLAY_WIDTH as u32, DISPLAY_HEIGHT as u32),
);
