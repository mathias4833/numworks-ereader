use embedded_graphics::prelude::*;
use embedded_graphics::primitives::Rectangle;

#[derive(Clone, Copy)]
pub enum StackDirection {
    Vertical,
    Horizontal,
}

#[derive(Clone, Copy)]
pub struct Stack {
    pub area: Rectangle,
    pub direction: StackDirection,
    pub item_size: u32,
    pub gap: u32,
}

impl Stack {
    pub const fn vertical(area: Rectangle, item_height: u32, gap: u32) -> Self {
        Self {
            area,
            direction: StackDirection::Vertical,
            item_size: item_height,
            gap,
        }
    }

    pub const fn horizontal(area: Rectangle, item_width: u32, gap: u32) -> Self {
        Self {
            area,
            direction: StackDirection::Horizontal,
            item_size: item_width,
            gap,
        }
    }

    pub fn item_rect(&self, index: usize) -> Option<Rectangle> {
        let step = self.item_size.saturating_add(self.gap);
        let offset = (index as u32).saturating_mul(step);

        let rect = match self.direction {
            StackDirection::Vertical => Rectangle::new(
                self.area.top_left + Point::new(0, offset as i32),
                Size::new(self.area.size.width, self.item_size),
            ),
            StackDirection::Horizontal => Rectangle::new(
                self.area.top_left + Point::new(offset as i32, 0),
                Size::new(self.item_size, self.area.size.height),
            ),
        };

        if self.area.intersection(&rect) != rect {
            return None;
        }

        Some(rect)
    }

    pub const fn capacity(&self) -> usize {
        if self.item_size == 0 {
            return 0;
        }

        let step = self.item_size.saturating_add(self.gap);

        if step == 0 {
            return 0;
        }

        let available = match self.direction {
            StackDirection::Vertical => self.area.size.height,
            StackDirection::Horizontal => self.area.size.width,
        };

        let total = available.saturating_add(self.gap);
        total.saturating_div(step) as usize
    }
}
