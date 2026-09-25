use crate::ui::components::text_block::TextBlock;
use crate::ui::layout::Stack;
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
use embedded_graphics::text::renderer::TextRenderer;
use embedded_graphics::text::{Alignment, Baseline, Text, TextStyleBuilder};
use embedded_iconoir::prelude::{IconoirNewIcon, icons};
use heapless::String;

const COVER_TO_TEXT: i32 = 10;
const TEXT_TO_CHEVRON: i32 = 5;
const TEXT_GAP: u32 = 4;

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

        let chevron_position = Point::new(
            self.area.top_left.x + self.area.size.width as i32 - 29,
            self.area.top_left.y + (self.area.size.height as i32 - 18) / 2,
        );
        let text_origin = Point::new(
            cover.top_left.x + cover.size.width as i32 + COVER_TO_TEXT,
            self.area.top_left.y,
        );
        let text_width = (chevron_position.x - TEXT_TO_CHEVRON - text_origin.x) as u32;
        let text_area = Rectangle::new(text_origin, Size::new(text_width, self.area.size.height));
        let title = TextBlock::new(
            self.book.title(),
            text_width,
            theme::large_text(theme::FOREGROUND),
        );

        let secondary_style = theme::text(theme::SECONDARY_FOREGROUND);
        let line_height = secondary_style.line_height();
        let author = self.book.author();
        let secondary_lines = u32::from(self.label.is_some()) + u32::from(!author.is_empty());
        let block_height = title.height() + secondary_lines * (line_height + TEXT_GAP);

        let block = Rectangle::new(
            text_area.top_left
                + Point::new(0, (text_area.size.height as i32 - block_height as i32) / 2),
            Size::new(text_area.size.width, block_height),
        );
        let mut layout = Stack::vertical(block, TEXT_GAP);
        let mut clipped = display.clipped(&text_area);

        if let Some(label) = self.label {
            Text::with_baseline(
                label,
                layout.next(line_height).top_left,
                &secondary_style,
                Baseline::Top,
            )
            .draw(&mut clipped)?;
        }

        let title_area = layout.next(title.height());
        title.at(title_area.top_left).draw(&mut clipped)?;

        if !author.is_empty() {
            Text::with_baseline(
                author,
                layout.next(line_height).top_left,
                &secondary_style,
                Baseline::Top,
            )
            .draw(&mut clipped)?;
        }

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

        Image::new(
            &icons::size18px::navigation::NavArrowRight::new(theme::FOREGROUND),
            chevron_position,
        )
        .draw(display)?;

        Ok(())
    }
}
