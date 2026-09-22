use crate::app::AppAction;
use crate::eadk::event::Event;
use crate::screens::{Screen, ScreenContext};
use crate::ui::components::book_card::BookCard;
use crate::ui::components::menu::{Menu, MenuRow};
use crate::ui::components::status_bar::StatusBar;
use crate::ui::icons::Icon;
use crate::ui::layout::{Frame, Insets, SCREEN, Stack};
use crate::ui::theme;
use embedded_graphics::pixelcolor::Rgb565;
use embedded_graphics::prelude::*;
use embedded_graphics::primitives::PrimitiveStyle;

static HOME_ITEMS: [(&str, Icon); 2] = [("Library", Icon::Folder), ("Settings", Icon::Settings)];

pub struct HomeScreen {
    menu: Menu,
}

impl HomeScreen {
    pub const fn new() -> Self {
        Self {
            menu: Menu::new(HOME_ITEMS.len() + 1),
        }
    }
}

impl Screen for HomeScreen {
    fn draw<D>(&self, display: &mut D, ctx: ScreenContext<'_>) -> Result<(), D::Error>
    where
        D: DrawTarget<Color = Rgb565>,
    {
        let frame = Frame::new(SCREEN)
            .with_status_bar(StatusBar::HEIGHT)
            .with_padding(Insets::all(8));

        SCREEN
            .into_styled(PrimitiveStyle::with_fill(theme::BACKGROUND))
            .draw(display)?;

        StatusBar::new(frame.status_bar(), "NumWorks Reader")
            .with_battery(ctx.battery)
            .draw(display)?;

        let mut layout = Stack::vertical(frame.content(), 8);
        let book_panel = layout.next(BookCard::HEIGHT);

        if let Ok(Some(book)) = ctx.library.book(ctx.reading.current_book()) {
            BookCard::new(
                book_panel,
                &book,
                ctx.reading.progress_percent(book.page_count()),
            )
            .with_label("Current book")
            .selected(self.menu.selected() == 0)
            .draw(display)?;
        }

        for (index, &(label, icon)) in HOME_ITEMS.iter().enumerate() {
            MenuRow::new(layout.next(MenuRow::HEIGHT), label, icon)
                .selected(self.menu.selected() == index + 1)
                .draw(display)?;
        }

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
