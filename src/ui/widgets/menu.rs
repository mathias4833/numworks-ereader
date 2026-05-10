use crate::eadk::display::Color;
use crate::ui::draw;
use crate::ui::layout::Stack;
use core::ffi::CStr;

pub struct Menu<'a> {
    items: &'a [&'a CStr],
    selected: usize,
}

impl<'a> Menu<'a> {
    pub const fn new(items: &'a [&'a CStr]) -> Self {
        Self { items, selected: 0 }
    }

    pub fn selected(&self) -> usize {
        self.selected
    }

    pub fn move_up(&mut self) -> bool {
        let previous = self.selected;
        self.selected = self.selected.saturating_sub(1);
        self.selected != previous
    }

    pub fn move_down(&mut self) -> bool {
        if self.items.is_empty() {
            return false;
        }

        let previous = self.selected;
        self.selected = (self.selected + 1).min(self.items.len() - 1);
        self.selected != previous
    }

    pub fn draw(&self, layout: Stack) {
        for (index, item) in self.items.iter().enumerate() {
            let Some(area) = layout.item_rect(index) else {
                continue;
            };
            let selected = index == self.selected;

            let background = if selected { Color::BLACK } else { Color::WHITE };
            let foreground = if selected { Color::WHITE } else { Color::BLACK };

            draw::rect(area, background);
            draw::text(item, area.x + 8, area.y + 6, foreground, background);
        }
    }
}
