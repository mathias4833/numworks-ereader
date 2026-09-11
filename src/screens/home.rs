use crate::app::AppAction;
use crate::eadk::event::Event;
use crate::reading::ReadingState;
use crate::screens::{DrawContext, Screen};
use crate::ui::components::menu::Menu;
use crate::ui::components::status_bar::StatusBar;
use crate::ui::icons::{Icon, IconView};
use crate::ui::layout::{Frame, Insets, SCREEN, Stack};
use core::fmt::Write;
use embedded_graphics::mono_font::MonoTextStyle;
use embedded_graphics::mono_font::ascii::FONT_9X18;
use embedded_graphics::mono_font::jis_x0201::FONT_7X14;
use embedded_graphics::pixelcolor::Rgb565;
use embedded_graphics::prelude::*;
use embedded_graphics::primitives::{PrimitiveStyle, Rectangle};
use embedded_graphics::text::{Alignment, Baseline, Text, TextStyleBuilder};
use heapless::String;

static HOME_ITEMS: [&str; 4] = ["Open reader", "Browse files", "Recent books", "Settings"];
static HOME_ICONS: [Icon; 4] = [Icon::Book, Icon::Folder, Icon::Recent, Icon::Settings];

fn panel_gray() -> Rgb565 {
    Rgb565::new(26, 52, 26)
}

pub struct HomeScreen {
    menu: Menu<'static>,
}

impl HomeScreen {
    pub const fn new() -> Self {
        Self {
            menu: Menu::new(&HOME_ITEMS),
        }
    }
}

impl Screen for HomeScreen {
    fn draw<D>(&self, display: &mut D, ctx: DrawContext<'_>) -> Result<(), D::Error>
    where
        D: DrawTarget<Color = Rgb565>,
    {
        let frame = Frame::new(SCREEN)
            .with_status_bar(24)
            .with_padding(Insets::all(8));

        SCREEN
            .into_styled(PrimitiveStyle::with_fill(Rgb565::WHITE))
            .draw(display)?;

        StatusBar::new(frame.status_bar(), "NumWorks Reader")
            .with_battery(ctx.battery)
            .draw(display)?;

        let content = frame.content();
        let book_panel = Rectangle::new(content.top_left, Size::new(content.size.width, 74));
        draw_current_book(display, book_panel, ctx.reading)?;

        let menu_top = content.top_left.y + book_panel.size.height as i32 + 8;
        let menu_area = Rectangle::new(
            Point::new(content.top_left.x, menu_top),
            Size::new(
                content.size.width,
                content
                    .size
                    .height
                    .saturating_sub(book_panel.size.height + 8),
            ),
        );

        let menu_layout = Stack::vertical(menu_area, 22, 5);

        self.menu
            .view(menu_layout)
            .with_text_left_padding(28)
            .draw(display)?;

        draw_menu_icons(display, menu_layout, self.menu.selected())?;

        Ok(())
    }

    fn handle_event(&mut self, event: Event) -> AppAction {
        match event {
            Event::Up => AppAction::redraw_if(self.menu.move_up()),

            Event::Down => AppAction::redraw_if(self.menu.move_down()),

            Event::Ok => match self.menu.selected() {
                0 => AppAction::OpenReader,
                3 => AppAction::None, // TODO: Settings
                _ => AppAction::None,
            },

            _ => AppAction::None,
        }
    }
}

fn draw_current_book<D>(
    display: &mut D,
    area: Rectangle,
    reading: &ReadingState,
) -> Result<(), D::Error>
where
    D: DrawTarget<Color = Rgb565>,
{
    area.into_styled(PrimitiveStyle::with_fill(panel_gray()))
        .draw(display)?;

    let cover = Rectangle::new(area.top_left + Point::new(8, 8), Size::new(44, 58));
    cover
        .into_styled(PrimitiveStyle::with_fill(Rgb565::BLACK))
        .draw(display)?;

    Text::with_text_style(
        "NW",
        cover.center(),
        MonoTextStyle::new(&FONT_9X18, Rgb565::WHITE),
        TextStyleBuilder::new()
            .alignment(Alignment::Center)
            .baseline(Baseline::Middle)
            .build(),
    )
    .draw(display)?;

    Text::with_text_style(
        "Current book",
        area.top_left + Point::new(64, 20),
        MonoTextStyle::new(&FONT_7X14, Rgb565::BLACK),
        TextStyleBuilder::new()
            .alignment(Alignment::Left)
            .baseline(Baseline::Middle)
            .build(),
    )
    .draw(display)?;

    Text::with_text_style(
        reading.book().title(),
        area.top_left + Point::new(64, 40),
        MonoTextStyle::new(&FONT_7X14, Rgb565::BLACK),
        TextStyleBuilder::new()
            .alignment(Alignment::Left)
            .baseline(Baseline::Middle)
            .build(),
    )
    .draw(display)?;

    let mut progress = String::<5>::new();
    let _ = write!(progress, "{}%", reading.progress_percent());

    Text::with_text_style(
        "0%",
        area.top_left + Point::new(area.size.width as i32 - 12, 12),
        MonoTextStyle::new(&FONT_7X14, Rgb565::BLACK),
        TextStyleBuilder::new()
            .alignment(Alignment::Right)
            .baseline(Baseline::Middle)
            .build(),
    )
    .draw(display)?;

    Ok(())
}

fn draw_menu_icons<D>(display: &mut D, layout: Stack, selected_index: usize) -> Result<(), D::Error>
where
    D: DrawTarget<Color = Rgb565>,
{
    for index in 0..HOME_ITEMS.len() {
        let Some(area) = layout.item_rect(index) else {
            continue;
        };

        let color = if index == selected_index {
            Rgb565::WHITE
        } else {
            Rgb565::BLACK
        };
        let icon_area = Rectangle::new(area.top_left + Point::new(6, 5), Size::new(13, 12));

        IconView::new(HOME_ICONS[index], icon_area, color).draw(display)?;
    }

    Ok(())
}
