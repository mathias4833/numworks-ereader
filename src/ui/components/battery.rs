use crate::eadk;
use crate::eadk::battery::BatteryLevel;
use crate::eadk::event::Event;
use crate::ui::theme;
use embedded_graphics::image::Image;
use embedded_graphics::pixelcolor::Rgb565;
use embedded_graphics::prelude::*;
use embedded_graphics::primitives::Rectangle;
use embedded_iconoir::prelude::{IconoirNewIcon, icons};

const BATTERY_REFRESH_MS: u64 = 1000;

pub struct BatteryState {
    level: Option<BatteryLevel>,
    charging: bool,
    next_refresh_ms: u64,
}

impl BatteryState {
    pub const fn new() -> Self {
        Self {
            level: None,
            charging: false,
            next_refresh_ms: 0,
        }
    }

    pub fn level(&self) -> Option<BatteryLevel> {
        self.level
    }

    pub fn charging(&self) -> bool {
        self.charging
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

    pub fn view(&self, area: Rectangle) -> BatteryView<'_> {
        BatteryView::centered_in(area, self)
    }
}

pub struct BatteryView<'a> {
    state: &'a BatteryState,
    area: Rectangle,
}

impl<'a> BatteryView<'a> {
    pub const SIZE: Size = Size::new(18, 18);

    pub const fn new(area: Rectangle, state: &'a BatteryState) -> Self {
        Self { area, state }
    }

    pub fn centered_in(slot: Rectangle, state: &'a BatteryState) -> Self {
        let x = slot.top_left.x + (slot.size.width.saturating_sub(Self::SIZE.width) / 2) as i32;
        let y = slot.top_left.y + (slot.size.height.saturating_sub(Self::SIZE.height) / 2) as i32;

        Self::new(Rectangle::new(Point::new(x, y), Self::SIZE), state)
    }
}

impl Drawable for BatteryView<'_> {
    type Color = Rgb565;
    type Output = ();

    fn draw<D>(&self, display: &mut D) -> Result<(), D::Error>
    where
        D: DrawTarget<Color = Self::Color>,
    {
        let Some(level) = self.state.level() else {
            return Ok(());
        };

        let position = self.area.top_left;
        let color = theme::FOREGROUND;
        if self.state.charging() {
            Image::new(
                &icons::size18px::system::BatteryCharging::new(color),
                position,
            )
            .draw(display)?;
        } else {
            match level {
                BatteryLevel::Empty | BatteryLevel::Low => Image::new(
                    &icons::size18px::system::BatteryWarning::new(color),
                    position,
                )
                .draw(display)?,
                BatteryLevel::Percent40 => Image::new(
                    &icons::size18px::system::BatteryTwoFive::new(color),
                    position,
                )
                .draw(display)?,
                BatteryLevel::Percent60 => Image::new(
                    &icons::size18px::system::BatteryFiveZero::new(color),
                    position,
                )
                .draw(display)?,
                BatteryLevel::Percent80 => Image::new(
                    &icons::size18px::system::BatterySevenFive::new(color),
                    position,
                )
                .draw(display)?,
                BatteryLevel::Full => {
                    Image::new(&icons::size18px::system::BatteryFull::new(color), position)
                        .draw(display)?
                }
            }
        }

        Ok(())
    }
}
