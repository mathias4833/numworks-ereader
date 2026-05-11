use crate::app::AppAction;
use crate::screens::{DrawContext, Event, Screen};
use embedded_graphics::pixelcolor::Rgb565;
use embedded_graphics::prelude::DrawTarget;

pub struct ReaderScreen {}

impl ReaderScreen {
    pub const fn new() -> Self {
        Self {}
    }
}

impl Screen for ReaderScreen {
    fn draw<D>(&self, display: &mut D, ctx: DrawContext<'_>) -> Result<(), D::Error>
    where
        D: DrawTarget<Color = Rgb565>,
    {
        // TODO
        Ok(())
    }

    fn handle_event(&mut self, event: Event) -> AppAction {
        match event {
            Event::Back => AppAction::GoHome,
            _ => AppAction::None,
        }
    }
}
