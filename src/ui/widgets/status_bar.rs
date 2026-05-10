use crate::eadk::display::Color;
use crate::geometry::Rect;
use crate::ui::draw;
use crate::ui::widgets::battery_indicator::BatteryIndicator;
use core::ffi::CStr;

pub struct StatusBar<'a> {
    title: &'a CStr,
    battery: Option<&'a BatteryIndicator>,
}

impl<'a> StatusBar<'a> {
    pub const fn new(title: &'a CStr) -> Self {
        Self {
            title,
            battery: None,
        }
    }

    pub const fn with_battery(mut self, battery: &'a BatteryIndicator) -> Self {
        self.battery = Some(battery);
        self
    }

    pub fn draw(&self, area: Rect) {
        draw::rect(area, Color::WHITE);

        // Bottom separator
        draw::rect(
            Rect::new(area.x, area.bottom().saturating_sub(1), area.width, 1),
            Color::BLACK,
        );

        draw::centered_large_text(
            self.title,
            area.x + area.width / 2,
            area.y + area.height / 2,
            Color::BLACK,
            Color::WHITE,
        );

        if let Some(battery) = self.battery {
            let battery_area = Rect::new(area.right().saturating_sub(28), area.y + 5, 22, 14);

            battery.draw(battery_area);
        }
    }
}
