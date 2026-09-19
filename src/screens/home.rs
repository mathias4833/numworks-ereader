use crate::app::AppAction;
use crate::eadk::event::Event;
use crate::screens::{Screen, ScreenContext};
use crate::ui::components::book_card::BookCard;
use crate::ui::components::menu::{Menu, MenuRow};
use crate::ui::components::status_bar::StatusBar;
use crate::ui::icons::Icon;
use crate::ui::layout::{Frame, Insets, SCREEN, Stack};
use embedded_graphics::pixelcolor::Rgb565;
use embedded_graphics::prelude::*;
use embedded_graphics::primitives::{PrimitiveStyle, Rectangle};

static HOME_ITEMS: [&str; 3] = ["Open reader", "Library", "Settings"];
static HOME_ICONS: [Icon; 3] = [Icon::Book, Icon::Folder, Icon::Settings];

pub struct HomeScreen {
    menu: Menu,
}

impl HomeScreen {
    pub const fn new() -> Self {
        Self {
            menu: Menu::new(HOME_ITEMS.len()),
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
        let book_panel = Rectangle::new(
            content.top_left,
            Size::new(content.size.width, BookCard::HEIGHT),
        );

        if let Ok(Some(book)) = ctx.library.book(ctx.reading.current_book()) {
            BookCard::new(
                book_panel,
                &book,
                ctx.reading.progress_percent(book.page_count()),
            )
            .with_label("Current book")
            .draw(display)?;
        }

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
            .draw_with(display, |display, slot| {
                MenuRow::new(slot.area, HOME_ITEMS[slot.index], HOME_ICONS[slot.index])
                    .selected(slot.selected)
                    .draw(display)
            })?;

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
