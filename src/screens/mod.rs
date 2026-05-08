pub mod home;
pub mod reader;

pub enum Event {
    Ok,
    None,
} // TODO: Implement

pub trait Screen {
    type Action;

    fn draw(&self);
    fn handle_event(&mut self, event: Event) -> Option<Self::Action>;
}
