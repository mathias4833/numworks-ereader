use crate::app::AppAction;
use crate::eadk::event::Event;
use crate::ui::widgets::battery_indicator::BatteryIndicator;

pub mod home;
pub mod reader;

pub struct DrawContext<'a> {
    pub battery: &'a BatteryIndicator,
}

pub trait Screen {
    fn draw(&self, ctx: DrawContext<'_>);
    fn handle_event(&mut self, event: Event) -> AppAction;
}
