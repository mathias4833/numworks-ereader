use crate::app::AppAction;
use crate::eadk::event::Event;
use crate::ui::components::battery::BatteryState;
use embedded_graphics::pixelcolor::Rgb565;
use embedded_graphics::prelude::DrawTarget;

pub mod home;
pub mod reader;

pub struct DrawContext<'a> {
    pub battery: &'a BatteryState,
}

pub trait Screen {
    fn draw<D>(&self, display: &mut D, ctx: DrawContext<'_>) -> Result<(), D::Error>
    where
        D: DrawTarget<Color = Rgb565>;

    fn handle_event(&mut self, event: Event) -> AppAction;
}
