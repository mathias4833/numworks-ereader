use crate::ui::layout::Stack;
use embedded_graphics::Drawable;
use embedded_graphics::draw_target::DrawTarget;
use embedded_graphics::mono_font::MonoTextStyle;
use embedded_graphics::mono_font::jis_x0201::FONT_7X14;
use embedded_graphics::pixelcolor::Rgb565;
use embedded_graphics::prelude::*;
use embedded_graphics::primitives::{PrimitiveStyle, Rectangle};
use embedded_graphics::text::{Alignment, Baseline, Text, TextStyleBuilder};

pub struct Menu<I>
where
    I: AsRef<[&'static str]>,
{
    items: I,
    selected: usize,
}

impl<I> Menu<I>
where
    I: AsRef<[&'static str]>,
{
    pub const fn new(items: I) -> Self {
        Self { items, selected: 0 }
    }

    pub const fn with_selected(items: I, selected: usize) -> Self {
        Self { items, selected }
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
        if self.items.as_ref().is_empty() {
            return false;
        }

        let previous = self.selected;
        self.selected = (self.selected + 1).min(self.items.as_ref().len() - 1);
        self.selected != previous
    }

    pub fn view(&self, layout: Stack) -> MenuView<'_, I> {
        MenuView {
            menu: self,
            layout,
            text_left_padding: 8,
        }
    }
}

pub struct MenuView<'a, I>
where
    I: AsRef<[&'static str]>,
{
    menu: &'a Menu<I>,
    layout: Stack,
    text_left_padding: i32,
}

impl<I> MenuView<'_, I>
where
    I: AsRef<[&'static str]>,
{
    pub const fn with_text_left_padding(mut self, padding: i32) -> Self {
        self.text_left_padding = padding;
        self
    }
}

impl<I> Drawable for MenuView<'_, I>
where
    I: AsRef<[&'static str]>,
{
    type Color = Rgb565;
    type Output = ();

    fn draw<D>(&self, display: &mut D) -> Result<Self::Output, D::Error>
    where
        D: DrawTarget<Color = Self::Color>,
    {
        for (index, item) in self.menu.items.as_ref().iter().enumerate() {
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

            if selected {
                Rectangle::new(area.top_left, area.size)
                    .into_styled(PrimitiveStyle::with_fill(background))
                    .draw(display)?;
            }

            Text::with_text_style(
                item,
                Point::new(area.top_left.x + self.text_left_padding, area.center().y),
                MonoTextStyle::new(&FONT_7X14, foreground),
                TextStyleBuilder::new()
                    .alignment(Alignment::Left)
                    .baseline(Baseline::Middle)
                    .build(),
            )
            .draw(display)?;
        }

        Ok(())
    }
}
