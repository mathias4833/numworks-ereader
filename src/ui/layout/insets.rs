use embedded_graphics::{prelude::*, primitives::Rectangle};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Insets {
    pub top: u32,
    pub right: u32,
    pub bottom: u32,
    pub left: u32,
}

impl Insets {
    pub const ZERO: Self = Self::all(0);

    pub const fn new(top: u32, right: u32, bottom: u32, left: u32) -> Self {
        Self {
            top,
            right,
            bottom,
            left,
        }
    }

    pub const fn all(value: u32) -> Self {
        Self {
            top: value,
            right: value,
            bottom: value,
            left: value,
        }
    }

    pub const fn apply(self, area: Rectangle) -> Rectangle {
        Rectangle::new(
            Point::new(
                area.top_left.x + self.left as i32,
                area.top_left.y + self.top as i32,
            ),
            Size::new(
                area.size.width.saturating_sub(self.left + self.right),
                area.size.height.saturating_sub(self.top + self.bottom),
            ),
        )
    }
}
