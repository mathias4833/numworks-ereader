use crate::eadk;
use crate::eadk::battery::BatteryLevel;
use crate::eadk::event::Event;
use crate::ui::theme;
use embedded_graphics::pixelcolor::Rgb565;
use embedded_graphics::prelude::*;
use embedded_graphics::primitives::{Line, PrimitiveStyle, Rectangle, RoundedRectangle};

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
    pub const SIZE: Size = Size::new(24, 12);

    pub const fn new(area: Rectangle, state: &'a BatteryState) -> Self {
        Self { area, state }
    }

    pub fn centered_in(slot: Rectangle, state: &'a BatteryState) -> Self {
        let x = slot.top_left.x + (slot.size.width.saturating_sub(Self::SIZE.width) / 2) as i32;
        let y = slot.top_left.y + (slot.size.height.saturating_sub(Self::SIZE.height) / 2) as i32;

        Self::new(Rectangle::new(Point::new(x, y), Self::SIZE), state)
    }

    fn body(&self) -> Rectangle {
        // Reserve 2 px on the right for the nub.
        Rectangle::new(
            self.area.top_left,
            Size::new(Self::SIZE.width.saturating_sub(2), Self::SIZE.height),
        )
    }

    fn inner(&self) -> Rectangle {
        let body = self.body();

        Rectangle::new(
            body.top_left + Point::new(2, 3),
            Size::new(
                body.size.width.saturating_sub(4),
                body.size.height.saturating_sub(6),
            ),
        )
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

        self.draw_fill(display, level)?;
        self.draw_body(display)?;
        self.draw_nub(display)?;

        if self.state.charging() {
            self.draw_charging(display)?;
        }

        Ok(())
    }
}

impl BatteryView<'_> {
    fn draw_fill<D>(&self, display: &mut D, level: BatteryLevel) -> Result<(), D::Error>
    where
        D: DrawTarget<Color = Rgb565>,
    {
        let body = self.body();

        let level_raw: u8 = level.into();
        let width = body.size.width.saturating_mul(level_raw as u32) / 5;

        RoundedRectangle::with_equal_corners(
            Rectangle::new(body.top_left, Size::new(width, body.size.height)),
            Size::new(2, 2),
        )
        .into_styled(PrimitiveStyle::with_fill(color_for_level(level)))
        .draw(display)?;

        Ok(())
    }

    fn draw_body<D>(&self, display: &mut D) -> Result<(), D::Error>
    where
        D: DrawTarget<Color = Rgb565>,
    {
        RoundedRectangle::with_equal_corners(self.body(), Size::new(2, 2))
            .into_styled(PrimitiveStyle::with_stroke(theme::FOREGROUND, 1))
            .draw(display)?;

        Ok(())
    }

    fn draw_nub<D>(&self, display: &mut D) -> Result<(), D::Error>
    where
        D: DrawTarget<Color = Rgb565>,
    {
        let body = self.body();

        Rectangle::new(
            Point::new(
                body.top_left.x + body.size.width as i32,
                body.top_left.y + body.size.height as i32 / 3,
            ),
            Size::new(2, body.size.height / 3),
        )
        .into_styled(PrimitiveStyle::with_fill(theme::FOREGROUND))
        .draw(display)?;

        Ok(())
    }

    fn draw_charging<D>(&self, display: &mut D) -> Result<(), D::Error>
    where
        D: DrawTarget<Color = Rgb565>,
    {
        let inner = self.inner();

        let center_x = inner.top_left.x + inner.size.width as i32 / 2;
        let center_y = inner.top_left.y + inner.size.height as i32 / 2;

        let style = PrimitiveStyle::with_stroke(theme::FOREGROUND, 1);

        // Small lightning bolt.
        Line::new(
            Point::new(center_x + 2, center_y - 4),
            Point::new(center_x - 1, center_y),
        )
        .into_styled(style)
        .draw(display)?;

        Line::new(
            Point::new(center_x - 1, center_y),
            Point::new(center_x + 2, center_y),
        )
        .into_styled(style)
        .draw(display)?;

        Line::new(
            Point::new(center_x + 2, center_y),
            Point::new(center_x - 2, center_y + 4),
        )
        .into_styled(style)
        .draw(display)?;

        Ok(())
    }
}

fn color_for_level(level: BatteryLevel) -> Rgb565 {
    match level {
        BatteryLevel::Empty | BatteryLevel::Low => theme::DANGER,
        BatteryLevel::Percent40 => theme::WARNING,
        BatteryLevel::Percent60 | BatteryLevel::Percent80 | BatteryLevel::Full => theme::SUCCESS,
    }
}
