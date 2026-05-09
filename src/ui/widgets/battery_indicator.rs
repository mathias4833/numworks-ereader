use crate::eadk;
use crate::eadk::battery::BatteryLevel;
use crate::eadk::display::Rect;
use crate::eadk::event::Event;

const BATTERY_REFRESH_MS: u64 = 60_000;

pub struct BatteryIndicator {
    level: Option<BatteryLevel>,
    charging: bool,
    next_refresh_ms: u64,
}

impl BatteryIndicator {
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
        // TODO
    }
}
