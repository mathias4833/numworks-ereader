use crate::eadk::display::Rect;

pub struct Insets {
    pub left: u16,
    pub right: u16,
    pub top: u16,
    pub bottom: u16,
}

impl Insets {
    pub const fn new(left: u16, right: u16, top: u16, bottom: u16) -> Self {
        Self {
            left,
            right,
            top,
            bottom,
        }
    }

    pub const fn all(value: u16) -> Self {
        Self::new(value, value, value, value)
    }

    pub const fn symmetric(horizontal: u16, vertical: u16) -> Self {
        Self::new(horizontal, horizontal, vertical, vertical)
    }

    pub const fn apply(self, area: Rect) -> Rect {
        let left = area.left().saturating_add(self.left);
        let top = area.top().saturating_add(self.top);
        let right = area.right().saturating_sub(self.right);
        let bottom = area.bottom().saturating_sub(self.bottom);

        if right <= left || bottom <= top {
            return Rect::new(left, top, 0, 0);
        }

        Rect::new(left, top, right - left, bottom - top)
    }
}
