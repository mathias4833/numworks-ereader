use crate::ui::layout::{Stack, centered_line_y};
use crate::ui::theme;
use embedded_graphics::Drawable;
use embedded_graphics::draw_target::DrawTarget;
use embedded_graphics::image::Image;
use embedded_graphics::pixelcolor::Rgb565;
use embedded_graphics::prelude::{Point, Primitive};
use embedded_graphics::primitives::{Rectangle, RoundedRectangle};
use embedded_graphics::text::renderer::TextRenderer;
use embedded_graphics::text::{Alignment, Baseline, Text, TextStyleBuilder};
use embedded_iconoir::prelude::{IconoirNewIcon, icons};

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

    pub fn draw_with<D, F>(
        &self,
        display: &mut D,
        mut layout: Stack,
        item_size: u32,
        mut draw_item: F,
    ) -> Result<(), D::Error>
    where
        D: DrawTarget<Color = Rgb565>,
        F: FnMut(&mut D, MenuSlot) -> Result<(), D::Error>,
    {
        let capacity = layout.capacity(item_size);
        let first_visible = self.selected.saturating_sub(capacity.saturating_sub(1));

        for index in (first_visible..self.item_count).take(capacity) {
            draw_item(
                display,
                MenuSlot {
                    index,
                    area: layout.next(item_size),
                    selected: index == self.selected,
                },
            )?;
        }

        Ok(())
    }
}

pub struct MenuSlot {
    pub index: usize,
    pub area: Rectangle,
    pub selected: bool,
}

#[derive(Clone, Copy)]
pub enum MenuIcon {
    Folder,
    Settings,
}

pub struct MenuRow<'a> {
    area: Rectangle,
    text: &'a str,
    icon: MenuIcon,
    selected: bool,
}

impl<'a> MenuRow<'a> {
    pub const HEIGHT: u32 = 41;

    pub const fn new(area: Rectangle, text: &'a str, icon: MenuIcon) -> Self {
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
        RoundedRectangle::with_equal_corners(self.area, theme::CORNER_RADIUS)
            .into_styled(theme::surface(self.selected))
            .draw(display)?;

        let icon_y = self.area.top_left.y + (self.area.size.height as i32 - 18) / 2;
        let icon_position = Point::new(self.area.top_left.x + 11, icon_y);
        match self.icon {
            MenuIcon::Folder => Image::new(
                &icons::size18px::docs::Folder::new(theme::FOREGROUND),
                icon_position,
            )
            .draw(display)?,
            MenuIcon::Settings => Image::new(
                &icons::size18px::system::Settings::new(theme::FOREGROUND),
                icon_position,
            )
            .draw(display)?,
        }

        let chevron_position = Point::new(
            self.area.top_left.x + self.area.size.width as i32 - 29,
            icon_y,
        );
        Image::new(
            &icons::size18px::navigation::NavArrowRight::new(theme::FOREGROUND),
            chevron_position,
        )
        .draw(display)?;

        let style = theme::text(theme::FOREGROUND);
        let text_y = centered_line_y(self.area, style.line_height());
        Text::with_text_style(
            self.text,
            Point::new(self.area.top_left.x + 42, text_y),
            &style,
            TextStyleBuilder::new()
                .alignment(Alignment::Left)
                .baseline(Baseline::Top)
                .build(),
        )
        .draw(display)?;

        Ok(())
    }
}
