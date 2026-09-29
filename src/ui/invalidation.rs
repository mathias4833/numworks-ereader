use embedded_graphics::primitives::Rectangle;

#[derive(Clone, Copy, Default)]
pub enum Invalidation {
    #[default]
    None,
    Full,
    Partial {
        first: Rectangle,
        second: Option<Rectangle>,
    },
}

impl Invalidation {
    pub fn invalidate(&mut self, area: Rectangle) {
        *self = match *self {
            Self::None => Self::Partial {
                first: area,
                second: None,
            },
            Self::Partial {
                first,
                second: None,
            } => Self::Partial {
                first,
                second: Some(area),
            },
            Self::Partial { .. } | Self::Full => Self::Full,
        };
    }

    pub fn full(&mut self) {
        *self = Self::Full;
    }
}
