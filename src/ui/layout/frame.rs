use crate::geometry::Rect;
use crate::ui::layout::Insets;

#[derive(Clone, Copy)]
pub struct Frame {
    screen: Rect,
    status_bar_height: u16,
    bottom_bar_height: u16,
    padding: Insets,
}

impl Frame {
    pub const fn new(screen: Rect) -> Self {
        Self {
            screen,
            status_bar_height: 0,
            bottom_bar_height: 0,
            padding: Insets::ZERO,
        }
    }

    pub const fn with_status_bar(mut self, height: u16) -> Self {
        self.status_bar_height = height;
        self
    }

    pub const fn with_bottom_bar(mut self, height: u16) -> Self {
        self.bottom_bar_height = height;
        self
    }

    pub const fn with_padding(mut self, padding: Insets) -> Self {
        self.padding = padding;
        self
    }

    pub const fn status_bar(&self) -> Rect {
        Rect::new(
            self.screen.x,
            self.screen.y,
            self.screen.width,
            self.status_bar_height,
        )
    }

    pub const fn bottom_bar(&self) -> Rect {
        Rect::new(
            self.screen.x,
            self.screen.bottom().saturating_sub(self.bottom_bar_height),
            self.screen.width,
            self.bottom_bar_height,
        )
    }

    pub const fn body(&self) -> Rect {
        Rect::from_edges(
            self.screen.left(),
            self.status_bar().bottom(),
            self.screen.right(),
            self.bottom_bar().top(),
        )
    }

    pub const fn content(&self) -> Rect {
        self.padding.apply(self.body())
    }
}
