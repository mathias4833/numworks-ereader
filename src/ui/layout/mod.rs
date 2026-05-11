mod frame;
mod insets;
mod stack;

use crate::eadk::display::{DISPLAY_HEIGHT, DISPLAY_WIDTH};
use embedded_graphics::geometry::{Point, Size};
use embedded_graphics::primitives::Rectangle;
pub use frame::Frame;
pub use insets::Insets;
pub use stack::Stack;

pub const SCREEN: Rectangle = Rectangle::new(
    Point::new(0, 0),
    Size::new(DISPLAY_WIDTH as u32, DISPLAY_HEIGHT as u32),
);
