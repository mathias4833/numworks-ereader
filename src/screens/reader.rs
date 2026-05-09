use crate::eadk;
use crate::eadk::display::{Color, Point};
use crate::screens::{Event, Screen};

pub enum ReaderAction {}

pub struct ReaderScreen {}

impl ReaderScreen {
    pub const fn new() -> Self {
        Self {}
    }
}

impl Screen for ReaderScreen {
    type Action = ReaderAction;

    fn draw(&self) {
        eadk::display::draw_string(
            c"Reader!",
            Point::new(100, 100),
            true,
            Color::BLACK,
            Color::WHITE,
        );
    }

    fn handle_event(&mut self, _event: Event) -> Option<Self::Action> {
        None
    }
}
