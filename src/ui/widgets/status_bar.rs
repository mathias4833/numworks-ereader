use crate::eadk::display::{Color, Rect};
use crate::ui;
use crate::ui::draw;
use core::ffi::CStr;

pub struct StatusBar<'a> {
    title: &'a CStr,
}

impl<'a> StatusBar<'a> {
    pub const fn new(title: &'a CStr) -> Self {
        Self { title }
    }

    pub fn draw(&self, area: Rect) {
        ui::draw::rect(area, Color::WHITE);

        // Bottom separator
        ui::draw::rect(
            Rect::new(area.x, area.bottom().saturating_sub(1), area.width, 1),
            Color::BLACK,
        );

        self.draw_title(area)
    }

    fn draw_title(&self, area: Rect) {
        draw::centered_large_text(
            self.title,
            area.x + area.width / 2,
            area.y + area.height / 2,
            Color::BLACK,
            Color::WHITE,
        )
    }
}
