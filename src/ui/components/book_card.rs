use book_format::{Book, COVER_HEIGHT, COVER_WIDTH};
use core::fmt::Write;
use embedded_graphics::Drawable;
use embedded_graphics::geometry::{Point, Size};
use embedded_graphics::mono_font::MonoTextStyle;
use embedded_graphics::mono_font::jis_x0201::FONT_7X14;
use embedded_graphics::pixelcolor::Rgb565;
use embedded_graphics::pixelcolor::raw::RawU16;
use embedded_graphics::prelude::{DrawTarget, Primitive, RgbColor};
use embedded_graphics::primitives::{PrimitiveStyle, Rectangle};
use embedded_graphics::text::{Alignment, Baseline, Text, TextStyleBuilder};
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
        let background = if self.selected {
            Rgb565::BLACK
        } else {
            // Gray
            Rgb565::new(26, 52, 26)
        };
        let foreground = if self.selected {
            Rgb565::WHITE
        } else {
            Rgb565::BLACK
        };

        self.area
            .into_styled(PrimitiveStyle::with_fill(background))
            .draw(display)?;

        let cover = Rectangle::new(
            self.area.top_left + Point::new(8, 5),
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
                MonoTextStyle::new(&FONT_7X14, foreground),
                TextStyleBuilder::new()
                    .alignment(Alignment::Left)
                    .baseline(Baseline::Middle)
                    .build(),
            )
            .draw(display)?;
        }

        let title_y = if self.label.is_some() { 59 } else { 50 };
        Text::with_text_style(
            self.book.title(),
            self.area.top_left + Point::new(80, title_y),
            MonoTextStyle::new(&FONT_7X14, foreground),
            TextStyleBuilder::new()
                .alignment(Alignment::Left)
                .baseline(Baseline::Middle)
                .build(),
        )
        .draw(display)?;

        let mut progress = String::<5>::new();
        let _ = write!(progress, "{}%", self.progress);

        Text::with_text_style(
            progress.as_str(),
            self.area.top_left + Point::new(self.area.size.width as i32 - 12, 12),
            MonoTextStyle::new(&FONT_7X14, foreground),
            TextStyleBuilder::new()
                .alignment(Alignment::Right)
                .baseline(Baseline::Middle)
                .build(),
        )
        .draw(display)?;

        Ok(())
    }
}
