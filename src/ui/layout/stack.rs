use crate::geometry::Rect;

#[derive(Clone, Copy)]
pub enum StackDirection {
    Vertical,
    Horizontal,
}

#[derive(Clone, Copy)]
pub struct Stack {
    pub area: Rect,
    pub direction: StackDirection,
    pub item_size: u16,
    pub gap: u16,
}

impl Stack {
    pub const fn vertical(area: Rect, item_height: u16, gap: u16) -> Self {
        Self {
            area,
            direction: StackDirection::Vertical,
            item_size: item_height,
            gap,
        }
    }

    pub const fn horizontal(area: Rect, item_width: u16, gap: u16) -> Self {
        Self {
            area,
            direction: StackDirection::Horizontal,
            item_size: item_width,
            gap,
        }
    }

    pub const fn item_rect(&self, index: usize) -> Option<Rect> {
        let step = self.item_size.saturating_add(self.gap);
        let offset = (index as u16).saturating_mul(step);

        let rect = match self.direction {
            StackDirection::Vertical => Rect::new(
                self.area.x,
                self.area.y.saturating_add(offset),
                self.area.width,
                self.item_size,
            ),

            StackDirection::Horizontal => Rect::new(
                self.area.x.saturating_add(offset),
                self.area.y,
                self.item_size,
                self.area.height,
            ),
        };

        match self.direction {
            StackDirection::Vertical => {
                if rect.bottom() > self.area.bottom() {
                    return None;
                }
            }

            StackDirection::Horizontal => {
                if rect.right() > self.area.right() {
                    return None;
                }
            }
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
            StackDirection::Vertical => self.area.height,
            StackDirection::Horizontal => self.area.width,
        };

        let total = available.saturating_add(self.gap);
        total.saturating_div(step) as usize
    }
}
