use crate::eadk::event::Event;

pub mod home;
pub mod reader;

pub trait Screen {
    type Action;

    fn draw(&self);
    fn handle_event(&mut self, event: Event) -> Option<Self::Action>;
}
