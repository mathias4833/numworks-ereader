use crate::ui::layout::centered_line_y;
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
