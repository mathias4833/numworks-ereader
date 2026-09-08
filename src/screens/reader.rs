use crate::app::AppAction;
use crate::screens::{DrawContext, Event, Screen};
use crate::ui::components::status_bar::StatusBar;
use crate::ui::layout::{Frame, Insets, SCREEN};
use book_format::{Book, Page};
use core::fmt::Write;
use embedded_graphics::mono_font::MonoTextStyle;
use embedded_graphics::mono_font::jis_x0201::FONT_7X14;
use embedded_graphics::pixelcolor::Rgb565;
use embedded_graphics::prelude::*;
use embedded_graphics::primitives::{Line, PrimitiveStyle, Rectangle};
use embedded_graphics::text::{Alignment, Baseline, Text, TextStyleBuilder};
use heapless::String;

pub struct ReaderScreen {
    book: Book<'static>,
    page_index: usize,
}

impl ReaderScreen {
    pub const fn new(book: Book<'static>) -> Self {
        Self {
            book,
            page_index: 0,
        }
    }

    fn previous_page(&mut self) -> bool {
        if self.page_index == 0 {
            return false;
        }

        self.page_index -= 1;
        true
    }

    fn next_page(&mut self) -> bool {
        if self.page_index + 1 >= self.book.page_count() {
            return false;
        }

        self.page_index += 1;
        true
    }
}

impl Screen for ReaderScreen {
    fn draw<D>(&self, display: &mut D, ctx: DrawContext<'_>) -> Result<(), D::Error>
    where
        D: DrawTarget<Color = Rgb565>,
    {
        let frame = Frame::new(SCREEN)
            .with_status_bar(24)
            .with_bottom_bar(20)
            .with_padding(Insets::new(8, 3, 8, 3));

        SCREEN
            .into_styled(PrimitiveStyle::with_fill(Rgb565::WHITE))
            .draw(display)?;

        StatusBar::new(frame.status_bar(), self.book.title())
            .with_battery(ctx.battery)
            .draw(display)?;

        debug_assert!(self.page_index < self.book.page_count());

        if let Some(page) = self.book.page(self.page_index) {
            draw_reader_text(display, frame.content(), &page)?;
        }

        draw_bottom_bar(
            display,
            frame.bottom_bar(),
            self.page_index,
            self.book.page_count(),
        )?;

        Ok(())
    }

    fn handle_event(&mut self, event: Event) -> AppAction {
        match event {
            Event::Left | Event::Up => AppAction::redraw_if(self.previous_page()),
            Event::Right | Event::Down | Event::Ok => AppAction::redraw_if(self.next_page()),
            Event::Back => AppAction::GoHome,
            _ => AppAction::None,
        }
    }
}

fn draw_reader_text<D>(display: &mut D, area: Rectangle, page: &Page<'_>) -> Result<(), D::Error>
where
    D: DrawTarget<Color = Rgb565>,
{
    let line_height = FONT_7X14.character_size.height as i32 + 2;
    let style = MonoTextStyle::new(&FONT_7X14, Rgb565::BLACK);
    let text_style = TextStyleBuilder::new()
        .alignment(Alignment::Left)
        .baseline(Baseline::Top)
        .build();

    for (index, line) in page.lines().enumerate() {
        Text::with_text_style(
            line,
            area.top_left + Point::new(0, index as i32 * line_height),
            style,
            text_style,
        )
        .draw(display)?;
    }

    Ok(())
}

fn draw_bottom_bar<D>(
    display: &mut D,
    area: Rectangle,
    page_index: usize,
    page_count: usize,
) -> Result<(), D::Error>
where
    D: DrawTarget<Color = Rgb565>,
{
    let mut page_label = String::<16>::new();
    let _ = write!(page_label, "Page {} / {}", page_index + 1, page_count);

    let mut progress_label = String::<5>::new();
    let _ = write!(
        progress_label,
        "{}%",
        progress_percent(page_index, page_count)
    );

    area.into_styled(PrimitiveStyle::with_fill(Rgb565::WHITE))
        .draw(display)?;

    Line::new(
        area.top_left,
        Point::new(area.top_left.x + area.size.width as i32, area.top_left.y),
    )
    .into_styled(PrimitiveStyle::with_stroke(Rgb565::BLACK, 1))
    .draw(display)?;

    Text::with_text_style(
        page_label.as_str(),
        Point::new(area.top_left.x + 8, area.center().y),
        MonoTextStyle::new(&FONT_7X14, Rgb565::BLACK),
        TextStyleBuilder::new()
            .alignment(Alignment::Left)
            .baseline(Baseline::Middle)
            .build(),
    )
    .draw(display)?;

    Text::with_text_style(
        progress_label.as_str(),
        Point::new(
            area.top_left.x + area.size.width as i32 - 8,
            area.center().y,
        ),
        MonoTextStyle::new(&FONT_7X14, Rgb565::BLACK),
        TextStyleBuilder::new()
            .alignment(Alignment::Right)
            .baseline(Baseline::Middle)
            .build(),
    )
    .draw(display)?;

    Ok(())
}

fn progress_percent(page_index: usize, page_count: usize) -> usize {
    if page_count == 0 {
        return 0;
    }

    ((page_index + 1) * 100) / page_count
}
