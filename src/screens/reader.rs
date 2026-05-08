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
        todo!()
    }

    fn handle_event(&mut self, event: Event) -> Option<Self::Action> {
        None
    }
}
