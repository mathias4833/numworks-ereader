use crate::app::AppAction;
use crate::eadk::event::Event;

pub mod home;
pub mod reader;

pub trait Screen {
    fn draw(&self);
    fn handle_event(&mut self, event: Event) -> AppAction;
}
