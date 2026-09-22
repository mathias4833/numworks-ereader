use crate::ui::components::battery::BatteryState;
use crate::ui::theme;
use embedded_graphics::pixelcolor::Rgb565;
use embedded_graphics::prelude::*;
use embedded_graphics::primitives::{Line, PrimitiveStyle, Rectangle};
use embedded_graphics::text::{Alignment, Baseline, Text, TextStyleBuilder};

pub struct StatusBar<'a> {
    area: Rectangle,
    title: &'a str,
    battery: Option<&'a BatteryState>,
}

impl<'a> StatusBar<'a> {
    pub const HEIGHT: u32 = 24;

    pub const fn new(area: Rectangle, title: &'a str) -> Self {
        Self {
            area,
            title,
            battery: None,
        }
    }

    pub const fn with_battery(mut self, battery: &'a BatteryState) -> Self {
        self.battery = Some(battery);
        self
    }
}

impl<'a> Drawable for StatusBar<'a> {
    type Color = Rgb565;
    type Output = ();

    fn draw<D>(&self, display: &mut D) -> Result<Self::Output, D::Error>
    where
        D: DrawTarget<Color = Self::Color>,
    {
        self.area
            .into_styled(PrimitiveStyle::with_fill(theme::BACKGROUND))
            .draw(display)?;

        let bottom = self.area.top_left.y + self.area.size.height as i32 - 1;

        Line::new(
            Point::new(self.area.top_left.x, bottom),
            Point::new(self.area.top_left.x + self.area.size.width as i32, bottom),
        )
        .into_styled(PrimitiveStyle::with_stroke(theme::FOREGROUND, 1))
        .draw(display)?;

        let style = theme::title(theme::FOREGROUND);

        Text::with_text_style(
            self.title,
            self.area.center(),
            style,
            TextStyleBuilder::new()
                .alignment(Alignment::Center)
                .baseline(Baseline::Middle)
                .build(),
        )
        .draw(display)?;

        if let Some(battery) = self.battery {
            let battery_area = Rectangle::new(
                Point::new(
                    self.area.top_left.x + self.area.size.width as i32 - 32,
                    self.area.top_left.y,
                ),
                Size::new(32, self.area.size.height),
            );

            battery.view(battery_area).draw(display)?;
        }

        Ok(())
    }
}
