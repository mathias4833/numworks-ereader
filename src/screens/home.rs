use crate::app::AppAction;
use crate::eadk::event::Event;
use crate::screens::{Screen, ScreenContext};
use crate::ui::components::menu::Menu;
use crate::ui::components::status_bar::StatusBar;
use crate::ui::icons::{Icon, IconView};
use crate::ui::layout::{Frame, Insets, SCREEN, Stack};
use book_format::{COVER_HEIGHT, COVER_WIDTH};
use core::fmt::Write;
use embedded_graphics::mono_font::MonoTextStyle;
use embedded_graphics::mono_font::jis_x0201::FONT_7X14;
use embedded_graphics::pixelcolor::Rgb565;
use embedded_graphics::pixelcolor::raw::RawU16;
use embedded_graphics::prelude::*;
use embedded_graphics::primitives::{PrimitiveStyle, Rectangle};
use embedded_graphics::text::{Alignment, Baseline, Text, TextStyleBuilder};
use heapless::String;

static HOME_ITEMS: [&str; 3] = ["Open reader", "Library", "Settings"];
static HOME_ICONS: [Icon; 3] = [Icon::Book, Icon::Folder, Icon::Settings];

fn panel_gray() -> Rgb565 {
    Rgb565::new(26, 52, 26)
}

pub struct HomeScreen {
    menu: Menu<&'static [&'static str]>,
}

impl HomeScreen {
    pub const fn new() -> Self {
        Self {
            menu: Menu::new(&HOME_ITEMS),
        }
    }
}

impl Screen for HomeScreen {
    fn draw<D>(&self, display: &mut D, ctx: ScreenContext<'_>) -> Result<(), D::Error>
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
        let book_panel = Rectangle::new(content.top_left, Size::new(content.size.width, 101));
        draw_current_book(display, book_panel, ctx)?;

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
                1 => AppAction::OpenLibrary,
                2 => AppAction::None, // TODO: Settings
                _ => AppAction::None,
            },

            _ => AppAction::None,
        }
    }
}

fn draw_current_book<D>(
    display: &mut D,
    area: Rectangle,
    ctx: ScreenContext<'_>,
) -> Result<(), D::Error>
where
    D: DrawTarget<Color = Rgb565>,
{
    area.into_styled(PrimitiveStyle::with_fill(panel_gray()))
        .draw(display)?;

    let book = ctx.library.book(ctx.reading.current_book()).ok().flatten();
    let cover = Rectangle::new(
        area.top_left + Point::new(8, 5),
        Size::new(COVER_WIDTH as u32, COVER_HEIGHT as u32),
    );
    if let Some(book) = &book {
        let pixels = book
            .cover()
            .chunks_exact(2)
            .map(|bytes| Rgb565::from(RawU16::new(u16::from_le_bytes([bytes[0], bytes[1]]))));
        display.fill_contiguous(&cover, pixels)?;
    }

    Text::with_text_style(
        "Current book",
        area.top_left + Point::new(80, 32),
        MonoTextStyle::new(&FONT_7X14, Rgb565::BLACK),
        TextStyleBuilder::new()
            .alignment(Alignment::Left)
            .baseline(Baseline::Middle)
            .build(),
    )
    .draw(display)?;

    let title = book.as_ref().map(|book| book.title()).unwrap_or("Invalid");
    let page_count = book.as_ref().map(|book| book.page_count()).unwrap_or(0);

    Text::with_text_style(
        title,
        area.top_left + Point::new(80, 59),
        MonoTextStyle::new(&FONT_7X14, Rgb565::BLACK),
        TextStyleBuilder::new()
            .alignment(Alignment::Left)
            .baseline(Baseline::Middle)
            .build(),
    )
    .draw(display)?;

    let mut progress = String::<5>::new();
    let _ = write!(progress, "{}%", ctx.reading.progress_percent(page_count));

    Text::with_text_style(
        progress.as_str(),
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
