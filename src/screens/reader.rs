use crate::app::AppAction;
use crate::eadk;
use crate::eadk::display::Color;
use crate::geometry::Point;
use crate::screens::{DrawContext, Event, Screen};

pub struct ReaderScreen {}

impl ReaderScreen {
    pub const fn new() -> Self {
        Self {}
    }
}

impl Screen for ReaderScreen {
    fn draw(&self, ctx: DrawContext<'_>) {
        eadk::display::draw_string(
            c"Reader!",
            Point::new(100, 100),
            true,
            Color::BLACK,
            Color::WHITE,
        );
    }

    fn handle_event(&mut self, event: Event) -> AppAction {
        match event {
            Event::Back => AppAction::GoHome,
            _ => AppAction::None,
        }
    }
}
