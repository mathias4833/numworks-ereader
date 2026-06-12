use crate::app::AppAction;
use crate::screens::{DrawContext, Event, Screen};
use crate::ui::components::status_bar::StatusBar;
use crate::ui::layout::{Frame, Insets, SCREEN};
use embedded_graphics::mono_font::jis_x0201::FONT_7X14;
use embedded_graphics::mono_font::MonoTextStyle;
use embedded_graphics::pixelcolor::Rgb565;
use embedded_graphics::prelude::*;
use embedded_graphics::primitives::{Line, PrimitiveStyle, Rectangle};
use embedded_graphics::text::{Alignment, Baseline, Text, TextStyleBuilder};

const READER_LINES: [&str; 11] = [
    "Alice was beginning to get very tired of",
    "sitting by her sister on the bank, and of",
    "having nothing to do: once or twice she had",
    "peeped into the book her sister was reading,",
    "but it had no pictures or conversations in it,",
    "\"and what is the use of a book,\" thought",
    "Alice \"without pictures or conversations?\" So",
    "she was considering in her own mind (as well",
    "as she could, for the hot day made her feel",
    "very sleepy and stupid), whether the pleasure",
    "of making a daisy-chain would be worth the",
];

pub struct ReaderScreen {}

impl ReaderScreen {
    pub const fn new() -> Self {
        Self {}
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

        StatusBar::new(frame.status_bar(), "Alice in Wonderland")
            .with_battery(ctx.battery)
            .draw(display)?;

        draw_reader_text(display, frame.content())?;

        draw_bottom_bar(display, frame.bottom_bar())?;

        Ok(())
    }

    fn handle_event(&mut self, event: Event) -> AppAction {
        match event {
            Event::Back => AppAction::GoHome,
            _ => AppAction::None,
        }
    }
}

fn draw_reader_text<D>(display: &mut D, area: Rectangle) -> Result<(), D::Error>
where
    D: DrawTarget<Color = Rgb565>,
{
    let line_height = FONT_7X14.character_size.height as i32 + 2;
    let style = MonoTextStyle::new(&FONT_7X14, Rgb565::BLACK);
    let text_style = TextStyleBuilder::new()
        .alignment(Alignment::Left)
        .baseline(Baseline::Top)
        .build();

    for (index, line) in READER_LINES.iter().enumerate() {
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

fn draw_bottom_bar<D>(display: &mut D, area: Rectangle) -> Result<(), D::Error>
where
    D: DrawTarget<Color = Rgb565>,
{
    area.into_styled(PrimitiveStyle::with_fill(Rgb565::WHITE))
        .draw(display)?;

    Line::new(
        area.top_left,
        Point::new(area.top_left.x + area.size.width as i32, area.top_left.y),
    )
    .into_styled(PrimitiveStyle::with_stroke(Rgb565::BLACK, 1))
    .draw(display)?;

    Text::with_text_style(
        "Page 1 / 1",
        Point::new(area.top_left.x + 8, area.center().y),
        MonoTextStyle::new(&FONT_7X14, Rgb565::BLACK),
        TextStyleBuilder::new()
            .alignment(Alignment::Left)
            .baseline(Baseline::Middle)
            .build(),
    )
    .draw(display)?;

    Text::with_text_style(
        "100%",
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
