use crate::ui::layout::Insets;
use embedded_graphics::prelude::*;
use embedded_graphics::primitives::Rectangle;

#[derive(Clone, Copy)]
pub struct Frame {
    screen: Rectangle,
    status_bar_height: u32,
    bottom_bar_height: u32,
    padding: Insets,
}

impl Frame {
    pub const fn new(screen: Rectangle) -> Self {
        Self {
            screen,
            status_bar_height: 0,
            bottom_bar_height: 0,
            padding: Insets::ZERO,
        }
    }

    pub const fn with_status_bar(mut self, height: u32) -> Self {
        self.status_bar_height = height;
        self
    }

    pub const fn with_bottom_bar(mut self, height: u32) -> Self {
        self.bottom_bar_height = height;
        self
    }

    pub const fn with_padding(mut self, padding: Insets) -> Self {
        self.padding = padding;
        self
    }

    pub fn status_bar(&self) -> Rectangle {
        Rectangle::new(
            self.screen.top_left,
            Size::new(self.screen.size.width, self.status_bar_height),
        )
    }

    pub fn bottom_bar(&self) -> Rectangle {
        Rectangle::new(
            Point::new(
                self.screen.top_left.x,
                self.screen.top_left.y + self.screen.size.height as i32
                    - self.bottom_bar_height as i32,
            ),
            Size::new(self.screen.size.width, self.bottom_bar_height),
        )
    }

    pub fn body(&self) -> Rectangle {
        Rectangle::new(
            self.screen.top_left + Point::new(0, self.status_bar_height as i32),
            Size::new(
                self.screen.size.width,
                self.screen
                    .size
                    .height
                    .saturating_sub(self.status_bar_height + self.bottom_bar_height),
            ),
        )
    }

    pub fn content(&self) -> Rectangle {
        self.padding.apply(self.body())
    }
}
