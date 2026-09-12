use crate::app::AppAction;
use crate::eadk::event::Event;
use crate::screens::{Screen, ScreenContext};
use crate::ui::components::menu::Menu;
use crate::ui::components::status_bar::StatusBar;
use crate::ui::layout::{Frame, Insets, SCREEN, Stack};
use book_format::Library;
use embedded_graphics::Drawable;
use embedded_graphics::draw_target::DrawTarget;
use embedded_graphics::pixelcolor::Rgb565;
use embedded_graphics::prelude::{Primitive, RgbColor};
use embedded_graphics::primitives::PrimitiveStyle;
use heapless::Vec;

const MAX_BOOKS: usize = 32;

pub struct LibraryScreen {
    menu: Menu<Vec<&'static str, MAX_BOOKS>>,
}

impl LibraryScreen {
    pub fn new(library: &Library<'static>, selected: usize) -> Self {
        let mut items = Vec::new();
        for index in 0..library.book_count() {
            let title = library
                .book(index)
                .ok()
                .flatten()
                .map(|book| book.title())
                .unwrap_or("Invalid");

            if items.push(title).is_err() {
                break;
            }
        }

        Self {
            menu: Menu::with_selected(items, selected),
        }
    }
}

impl Screen for LibraryScreen {
    fn draw<D>(&self, display: &mut D, ctx: ScreenContext<'_>) -> Result<(), D::Error>
    where
        D: DrawTarget<Color = Rgb565>,
    {
        let frame = Frame::new(SCREEN)
            .with_status_bar(24)
            .with_padding(Insets::new(8, 3, 8, 3));

        SCREEN
            .into_styled(PrimitiveStyle::with_fill(Rgb565::WHITE))
            .draw(display)?;

        StatusBar::new(frame.status_bar(), "Library")
            .with_battery(ctx.battery)
            .draw(display)?;

        let layout = Stack::vertical(frame.content(), 22, 4);
        self.menu.view(layout).draw(display)?;

        Ok(())
    }

    fn handle_event(&mut self, event: Event) -> AppAction {
        match event {
            Event::Up => AppAction::redraw_if(self.menu.move_up()),

            Event::Down => AppAction::redraw_if(self.menu.move_down()),

            Event::Ok => AppAction::OpenBook(self.menu.selected()),

            Event::Back => AppAction::GoHome,

            _ => AppAction::None,
        }
    }
}
