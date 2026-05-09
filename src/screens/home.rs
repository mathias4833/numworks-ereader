use crate::app::AppAction;
use crate::eadk;
use crate::eadk::display::{Color, Point, Rect};
use crate::eadk::event::Event;
use crate::screens::Screen;

#[derive(Default)]
pub struct HomeScreen {}

impl HomeScreen {
    pub const fn new() -> Self {
        Self {}
    }
}

impl Screen for HomeScreen {
    fn draw(&self) {
        eadk::display::fill_screen(Color::WHITE);

        eadk::display::push_rect_uniform(Rect::new(40, 40, 240, 160), Color::RED);

        eadk::display::draw_string(
            c"Hello World!",
            Point::new(100, 100),
            false,
            Color::BLACK,
            Color::WHITE,
        );
    }

    fn handle_event(&mut self, event: Event) -> AppAction {
        match event {
            Event::Ok => AppAction::OpenReader,
            _ => AppAction::None,
        }
    }
}
