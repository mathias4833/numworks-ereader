use embedded_graphics::prelude::*;
use embedded_graphics::primitives::Rectangle;

#[derive(Clone, Copy)]
pub struct Stack {
    pub area: Rectangle,
    pub gap: u32,
    offset: u32,
}

impl Stack {
    pub const fn vertical(area: Rectangle, gap: u32) -> Self {
        Self {
            area,
            gap,
            offset: 0,
        }
    }

    pub fn next(&mut self, size: u32) -> Rectangle {
        let rect = Rectangle::new(
            self.area.top_left + Point::new(0, self.offset as i32),
            Size::new(self.area.size.width, size),
        );

        self.offset = self.offset.saturating_add(size).saturating_add(self.gap);
        rect
    }
}
