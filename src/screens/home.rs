use crate::eadk::event::Event;
use crate::screens::{Route, Screen, ScreenContext, ScreenResult, UpdateContext};
use crate::ui::components::book_card::BookCard;
use crate::ui::components::list::List;
use crate::ui::components::menu_row::{MenuIcon, MenuRow};
use crate::ui::components::status_bar::StatusBar;
use crate::ui::layout::{Frame, Insets, SCREEN, Stack};
use crate::ui::theme;
use embedded_graphics::pixelcolor::Rgb565;
use embedded_graphics::prelude::*;
use embedded_graphics::primitives::PrimitiveStyle;

static HOME_ITEMS: [(&str, MenuIcon); 2] = [
    ("Library", MenuIcon::Folder),
    ("Settings", MenuIcon::Settings),
];

pub struct HomeScreen {
    frame: Frame,
    list: List<3>,
}

impl HomeScreen {
    pub fn new() -> Self {
        let frame = Frame::new(SCREEN)
            .with_status_bar(StatusBar::HEIGHT)
            .with_padding(Insets::all(8));
        let mut layout = Stack::vertical(frame.content(), 8);
        let slots = [
            layout.next(BookCard::HEIGHT),
            layout.next(MenuRow::HEIGHT),
            layout.next(MenuRow::HEIGHT),
        ];

        Self {
            frame,
            list: List::new(frame.content(), slots, HOME_ITEMS.len() + 1, 0),
        }
    }
}

impl Screen for HomeScreen {
    fn draw<D>(&self, display: &mut D, ctx: ScreenContext<'_>) -> Result<(), D::Error>
    where
        D: DrawTarget<Color = Rgb565>,
    {
        SCREEN
            .into_styled(PrimitiveStyle::with_fill(theme::BACKGROUND))
            .draw(display)?;

        StatusBar::new(self.frame.status_bar(), "NumWorks Reader")
            .with_battery(ctx.battery)
            .draw(display)?;

        for slot in self.list.slots() {
            if slot.index == 0 {
                if let Ok(Some(book)) = ctx.library.book(ctx.reading.current_book()) {
                    BookCard::new(
                        slot.area,
                        &book,
                        ctx.reading.progress_percent(book.page_count()),
                    )
                    .with_label("Current book")
                    .selected(slot.selected)
                    .draw(display)?;
                }

                continue;
            }

            let (label, icon) = HOME_ITEMS[slot.index - 1];
            MenuRow::new(slot.area, label, icon)
                .selected(slot.selected)
                .draw(display)?;
        }

        Ok(())
    }

    fn on_event(&mut self, event: Event, ctx: &mut UpdateContext<'_>) -> ScreenResult {
        match event {
            Event::Up => {
                self.list.move_up(ctx.invalidation);
                ScreenResult::None
            }

            Event::Down => {
                self.list.move_down(ctx.invalidation);
                ScreenResult::None
            }

            Event::Ok => match self.list.selected() {
                0 => ScreenResult::Navigate(Route::Reader),
                1 => ScreenResult::Navigate(Route::Library),
                2 => ScreenResult::None, // TODO: Settings
                _ => ScreenResult::None,
            },

            _ => ScreenResult::None,
        }
    }
}
