use crate::ui::layout::enclosing_rect;
use embedded_graphics::primitives::Rectangle;

#[derive(Clone, Copy, Default)]
pub enum Invalidation {
    #[default]
    None,
    Full,
    Partial(Rectangle),
}

impl Invalidation {
    pub fn invalidate(&mut self, area: Rectangle) {
        *self = match *self {
            Self::None => Self::Partial(area),
            Self::Partial(current) => Self::Partial(enclosing_rect(current, area)),
            Self::Full => Self::Full,
        };
    }

    pub fn full(&mut self) {
        *self = Self::Full;
    }
}
