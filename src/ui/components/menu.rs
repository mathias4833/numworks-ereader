use crate::ui::icons::{Icon, IconView};
use crate::ui::layout::Stack;
use crate::ui::theme;
use embedded_graphics::Drawable;
use embedded_graphics::draw_target::DrawTarget;
use embedded_graphics::pixelcolor::Rgb565;
use embedded_graphics::prelude::{Point, Primitive, Size};
use embedded_graphics::primitives::{PrimitiveStyle, Rectangle};
use embedded_graphics::text::{Alignment, Baseline, Text, TextStyleBuilder};

pub struct Menu {
    item_count: usize,
    selected: usize,
}

impl Menu {
    pub const fn new(item_count: usize) -> Self {
        Self {
            item_count,
            selected: 0,
        }
    }

    pub const fn with_selected(item_count: usize, selected: usize) -> Self {
        Self {
            item_count,
            selected,
        }
    }

    pub const fn selected(&self) -> usize {
        self.selected
    }

    pub fn move_up(&mut self) -> bool {
        let previous = self.selected;
        self.selected = self.selected.saturating_sub(1);
        self.selected != previous
    }

    pub fn move_down(&mut self) -> bool {
        if self.item_count == 0 {
            return false;
        }

        let previous = self.selected;
        self.selected = (self.selected + 1).min(self.item_count - 1);
        self.selected != previous
    }

    pub const fn view(&self, layout: Stack) -> MenuView<'_> {
        MenuView { menu: self, layout }
    }
}

pub struct MenuSlot {
    pub index: usize,
    pub area: Rectangle,
    pub selected: bool,
}

pub struct MenuView<'a> {
    menu: &'a Menu,
    layout: Stack,
}

impl MenuView<'_> {
    fn first_visible(&self) -> usize {
        let capacity = self.layout.capacity().max(1);
        self.menu.selected.saturating_sub(capacity - 1)
    }
}

impl MenuView<'_> {
    pub fn draw_with<D, F>(&self, display: &mut D, mut draw_item: F) -> Result<(), D::Error>
    where
        D: DrawTarget<Color = Rgb565>,
        F: FnMut(&mut D, MenuSlot) -> Result<(), D::Error>,
    {
        let first_visible = self.first_visible();

        for index in first_visible..self.menu.item_count {
            let slot = index - first_visible;
            let Some(area) = self.layout.item_rect(slot) else {
                continue;
            };

            draw_item(
                display,
                MenuSlot {
                    index,
                    area,
                    selected: index == self.menu.selected,
                },
            )?;
        }

        Ok(())
    }
}

pub struct MenuRow<'a> {
    area: Rectangle,
    text: &'a str,
    icon: Icon,
    selected: bool,
}

impl<'a> MenuRow<'a> {
    pub const HEIGHT: u32 = 22;

    pub const fn new(area: Rectangle, text: &'a str, icon: Icon) -> Self {
        Self {
            area,
            text,
            icon,
            selected: false,
        }
    }

    pub const fn selected(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }
}

impl Drawable for MenuRow<'_> {
    type Color = Rgb565;
    type Output = ();

    fn draw<D>(&self, display: &mut D) -> Result<(), D::Error>
    where
        D: DrawTarget<Color = Rgb565>,
    {
        let foreground = if self.selected {
            theme::SELECTED_FOREGROUND
        } else {
            theme::FOREGROUND
        };

        if self.selected {
            self.area
                .into_styled(PrimitiveStyle::with_fill(theme::SELECTED_BACKGROUND))
                .draw(display)?;
        }

        let icon_area = Rectangle::new(self.area.top_left + Point::new(6, 5), Size::new(13, 12));
        IconView::new(self.icon, icon_area, foreground).draw(display)?;

        Text::with_text_style(
            self.text,
            Point::new(self.area.top_left.x + 28, self.area.center().y),
            theme::text(foreground),
            TextStyleBuilder::new()
                .alignment(Alignment::Left)
                .baseline(Baseline::Middle)
                .build(),
        )
        .draw(display)?;

        Ok(())
    }
}
