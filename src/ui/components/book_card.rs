use crate::ui::theme;
use book_format::{Book, COVER_HEIGHT, COVER_WIDTH};
use core::fmt::Write;
use embedded_graphics::Drawable;
use embedded_graphics::geometry::{Point, Size};
use embedded_graphics::image::Image;
use embedded_graphics::pixelcolor::Rgb565;
use embedded_graphics::pixelcolor::raw::RawU16;
use embedded_graphics::prelude::{DrawTarget, DrawTargetExt, Primitive};
use embedded_graphics::primitives::{Rectangle, RoundedRectangle};
use embedded_graphics::text::{Alignment, Baseline, Text, TextStyleBuilder};
use embedded_iconoir::prelude::{IconoirNewIcon, icons};
use heapless::String;

pub struct BookCard<'a> {
    area: Rectangle,
    book: &'a Book<'a>,
    progress: usize,
    label: Option<&'a str>,
    selected: bool,
}

impl<'a> BookCard<'a> {
    pub const HEIGHT: u32 = 101;

    pub const fn new(area: Rectangle, book: &'a Book<'a>, progress: usize) -> Self {
        Self {
            area,
            book,
            progress,
            label: None,
            selected: false,
        }
    }

    pub const fn with_label(mut self, label: &'a str) -> Self {
        self.label = Some(label);
        self
    }

    pub const fn selected(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }
}

impl<'a> Drawable for BookCard<'a> {
    type Color = Rgb565;
    type Output = ();

    fn draw<D>(&self, display: &mut D) -> Result<(), D::Error>
    where
        D: DrawTarget<Color = Rgb565>,
    {
        RoundedRectangle::with_equal_corners(self.area, theme::CORNER_RADIUS)
            .into_styled(theme::surface(self.selected))
            .draw(display)?;

        let cover = Rectangle::new(
            self.area.top_left + Point::new(10, 5),
            Size::new(COVER_WIDTH as u32, COVER_HEIGHT as u32),
        );
        let pixels = self
            .book
            .cover()
            .chunks_exact(2)
            .map(|bytes| Rgb565::from(RawU16::new(u16::from_le_bytes([bytes[0], bytes[1]]))));

        display.fill_contiguous(&cover, pixels)?;

        if let Some(label) = self.label {
            Text::with_text_style(
                label,
                self.area.top_left + Point::new(80, 32),
                theme::text(theme::FOREGROUND),
                TextStyleBuilder::new()
                    .alignment(Alignment::Left)
                    .baseline(Baseline::Middle)
                    .build(),
            )
            .draw(display)?;
        }

        let title_y = if self.label.is_some() { 58 } else { 44 };
        let title_area = Rectangle::new(
            self.area.top_left + Point::new(80, title_y - 10),
            Size::new(self.area.size.width.saturating_sub(114), 20),
        );
        Text::with_text_style(
            self.book.title(),
            self.area.top_left + Point::new(80, title_y),
            theme::text(theme::FOREGROUND),
            TextStyleBuilder::new()
                .alignment(Alignment::Left)
                .baseline(Baseline::Middle)
                .build(),
        )
        .draw(&mut display.clipped(&title_area))?;

        let author_y = if self.label.is_some() { 80 } else { 68 };
        let author_area = Rectangle::new(
            self.area.top_left + Point::new(80, author_y - 10),
            Size::new(self.area.size.width.saturating_sub(114), 20),
        );
        Text::with_text_style(
            self.book.author(),
            self.area.top_left + Point::new(80, author_y),
            theme::text(theme::FOREGROUND),
            TextStyleBuilder::new()
                .alignment(Alignment::Left)
                .baseline(Baseline::Middle)
                .build(),
        )
        .draw(&mut display.clipped(&author_area))?;

        let mut progress = String::<5>::new();
        let _ = write!(progress, "{}%", self.progress);

        Text::with_text_style(
            progress.as_str(),
            self.area.top_left + Point::new(self.area.size.width as i32 - 10, 14),
            theme::text(theme::FOREGROUND),
            TextStyleBuilder::new()
                .alignment(Alignment::Right)
                .baseline(Baseline::Middle)
                .build(),
        )
        .draw(display)?;

        let chevron_position = Point::new(
            self.area.top_left.x + self.area.size.width as i32 - 29,
            self.area.top_left.y + (self.area.size.height as i32 - 18) / 2,
        );
        Image::new(
            &icons::size18px::navigation::NavArrowRight::new(theme::FOREGROUND),
            chevron_position,
        )
        .draw(display)?;

        Ok(())
    }
}
