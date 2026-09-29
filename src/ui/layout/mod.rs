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

pub fn enclosing_rect(a: Rectangle, b: Rectangle) -> Rectangle {
    if a.is_zero_sized() {
        return b;
    }
    if b.is_zero_sized() {
        return a;
    }

    let top_left = a.top_left.component_min(b.top_left);
    let bottom_right = (a.top_left + a.size).component_max(b.top_left + b.size);

    Rectangle::new(
        top_left,
        Size::new(
            (bottom_right.x - top_left.x) as u32,
            (bottom_right.y - top_left.y) as u32,
        ),
    )
}

pub const SCREEN: Rectangle = Rectangle::new(
    Point::new(0, 0),
    Size::new(DISPLAY_WIDTH as u32, DISPLAY_HEIGHT as u32),
);
