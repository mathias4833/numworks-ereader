use crate::eadk;
use crate::eadk::battery::BatteryLevel;
use crate::eadk::display::Color;
use crate::eadk::event::Event;
use crate::geometry::{Rect, Size};
use crate::ui::draw;
use crate::ui::layout::Insets;

const BATTERY_REFRESH_MS: u64 = 60_000;

pub struct BatteryIndicator {
    level: Option<BatteryLevel>,
    charging: bool,
    next_refresh_ms: u64,
}

impl BatteryIndicator {
    pub const SIZE: Size = Size::new(24, 12);

    pub const fn new() -> Self {
        Self {
            level: None,
            charging: false,
            next_refresh_ms: 0,
        }
    }

    pub fn update_if_needed(&mut self, now_ms: u64, event: Event) -> bool {
        let due = now_ms >= self.next_refresh_ms;
        let event_related = matches!(event, Event::BatteryCharging | Event::USBPlug);

        if !due && !event_related {
            return false;
        }

        let level = eadk::battery::level();
        let charging = eadk::battery::is_charging();

        let changed = self.level != level || self.charging != charging;

        self.level = level;
        self.charging = charging;
        self.next_refresh_ms = now_ms.saturating_add(BATTERY_REFRESH_MS);

        changed
    }

    pub fn draw(&self, area: Rect) {
        let Some(level) = self.level else {
            return;
        };

        let area = area.centered_rect(Self::SIZE);

        self.draw_fill(area, level);
        self.draw_body(area);
        self.draw_nub(area);
    }

    fn draw_fill(&self, area: Rect, level: BatteryLevel) {
        let inner = Self::inner(area);

        let color = Self::color_for_level(level);
        let level: u8 = level.into();
        let width = inner.width.saturating_mul(level as u16) / 5;
        draw::rect(Rect::new(inner.x, inner.y, width, inner.height), color)
    }

    fn draw_body(&self, area: Rect) {
        draw::border(Self::body(area), Color::BLACK);
    }

    fn draw_nub(&self, area: Rect) {
        let body = Self::body(area);
        draw::rect(
            Rect::new(body.right(), body.y + body.height / 4, 2, body.height / 2),
            Color::BLACK,
        );
    }

    const fn body(area: Rect) -> Rect {
        // Reserve 2 px on the right for the nub.
        Insets::new(0, 2, 0, 0).apply(area)
    }

    const fn inner(area: Rect) -> Rect {
        Insets::all(1).apply(Self::body(area))
    }

    fn color_for_level(level: BatteryLevel) -> Color {
        match level {
            BatteryLevel::Empty | BatteryLevel::Low => Color::RED,
            BatteryLevel::Percent40 => Color::ORANGE,
            BatteryLevel::Percent60 | BatteryLevel::Percent80 | BatteryLevel::Full => Color::GREEN,
        }
    }
}
