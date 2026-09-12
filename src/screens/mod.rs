use crate::app::AppAction;
use crate::eadk::event::Event;
use crate::reading::ReadingState;
use crate::ui::components::battery::BatteryState;
use book_format::Library;
use embedded_graphics::pixelcolor::Rgb565;
use embedded_graphics::prelude::DrawTarget;

pub mod home;
pub mod library;
pub mod reader;

pub struct ScreenContext<'a> {
    pub battery: &'a BatteryState,
    pub reading: &'a ReadingState,
    pub library: &'a Library<'static>,
}

pub trait Screen {
    fn draw<D>(&self, display: &mut D, ctx: ScreenContext<'_>) -> Result<(), D::Error>
    where
        D: DrawTarget<Color = Rgb565>;

    fn handle_event(&mut self, event: Event) -> AppAction;
}
