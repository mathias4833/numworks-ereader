use crate::eadk::event::Event;
use crate::screens::{Route, Screen, ScreenContext, ScreenResult, UpdateContext};
use crate::ui::components::book_card::BookCard;
use crate::ui::components::list::List;
use crate::ui::components::status_bar::StatusBar;
use crate::ui::layout::{Frame, Insets, SCREEN, Stack};
use crate::ui::theme;
use book_format::Library;
use embedded_graphics::Drawable;
use embedded_graphics::draw_target::DrawTarget;
use embedded_graphics::pixelcolor::Rgb565;
use embedded_graphics::prelude::Primitive;
use embedded_graphics::primitives::PrimitiveStyle;

pub struct LibraryScreen {
    frame: Frame,
    list: List<3>,
}

impl LibraryScreen {
    pub fn new(library: &Library<'static>, selected: usize) -> Self {
        let frame = Frame::new(SCREEN)
            .with_status_bar(StatusBar::HEIGHT)
            .with_padding(Insets::new(8, 3, 0, 3));
        let mut layout = Stack::vertical(frame.content(), 4);
        let slots = [
            layout.next(BookCard::COMPACT_HEIGHT),
            layout.next(BookCard::COMPACT_HEIGHT),
            layout.next(BookCard::COMPACT_HEIGHT),
        ];

        Self {
            frame,
            list: List::new(frame.content(), slots, library.book_count(), selected),
        }
    }
}

impl Screen for LibraryScreen {
    fn draw<D>(&self, display: &mut D, ctx: ScreenContext<'_>) -> Result<(), D::Error>
    where
        D: DrawTarget<Color = Rgb565>,
    {
        SCREEN
            .into_styled(PrimitiveStyle::with_fill(theme::BACKGROUND))
            .draw(display)?;

        StatusBar::new(self.frame.status_bar(), "Library")
            .with_battery(ctx.battery)
            .draw(display)?;

        for slot in self.list.slots() {
            let Ok(Some(book)) = ctx.library.book(slot.index) else {
                continue;
            };

            let progress = ctx
                .reading
                .progress_percent_for(slot.index, book.page_count());

            BookCard::compact(slot.area, &book, progress)
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

            Event::Ok => {
                if ctx.reading.select_book(self.list.selected()) {
                    ScreenResult::Navigate(Route::Reader)
                } else {
                    ScreenResult::None
                }
            }

            Event::Back => ScreenResult::Navigate(Route::Home),

            _ => ScreenResult::None,
        }
    }
}
