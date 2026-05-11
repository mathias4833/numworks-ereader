use crate::ui::layout::Stack;
use embedded_graphics::draw_target::DrawTarget;
use embedded_graphics::mono_font::jis_x0201::FONT_7X14;
use embedded_graphics::mono_font::MonoTextStyle;
use embedded_graphics::pixelcolor::Rgb565;
use embedded_graphics::prelude::*;
use embedded_graphics::primitives::{PrimitiveStyle, Rectangle};
use embedded_graphics::text::{Alignment, Text};
use embedded_graphics::Drawable;

pub struct Menu<'a> {
    items: &'a [&'a str],
    selected: usize,
}

impl<'a> Menu<'a> {
    pub const fn new(items: &'a [&'a str]) -> Self {
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

    pub fn view(&self, layout: Stack) -> MenuView<'_, 'a> {
        MenuView { menu: self, layout }
    }
}

pub struct MenuView<'menu, 'items> {
    menu: &'menu Menu<'items>,
    layout: Stack,
}

impl Drawable for MenuView<'_, '_> {
    type Color = Rgb565;
    type Output = ();

    fn draw<D>(&self, display: &mut D) -> Result<Self::Output, D::Error>
    where
        D: DrawTarget<Color = Self::Color>,
    {
        for (index, item) in self.menu.items.iter().enumerate() {
            let Some(area) = self.layout.item_rect(index) else {
                continue;
            };
            let selected = index == self.menu.selected;

            let background = if selected {
                Rgb565::BLACK
            } else {
                Rgb565::WHITE
            };
            let foreground = if selected {
                Rgb565::WHITE
            } else {
                Rgb565::BLACK
            };

            Rectangle::new(area.top_left, area.size)
                .into_styled(PrimitiveStyle::with_fill(background))
                .draw(display)?;

            Text::with_alignment(
                item,
                area.top_left + Point::new(8, 6),
                MonoTextStyle::new(&FONT_7X14, foreground),
                Alignment::Left,
            )
            .draw(display)?;
        }

        Ok(())
    }
}
