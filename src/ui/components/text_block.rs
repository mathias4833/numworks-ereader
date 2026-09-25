use embedded_graphics::Drawable;
use embedded_graphics::geometry::{Point, Size};
use embedded_graphics::pixelcolor::Rgb565;
use embedded_graphics::prelude::{DrawTarget, DrawTargetExt};
use embedded_graphics::primitives::Rectangle;
use embedded_graphics::text::renderer::TextRenderer;
use embedded_graphics::text::{Baseline, Text};
use u8g2_fonts::U8g2TextStyle;

pub struct TextBlock<'a> {
    first: &'a str,
    second: Option<&'a str>,
    style: U8g2TextStyle<Rgb565>,
    width: u32,
    position: Point,
}

impl<'a> TextBlock<'a> {
    pub fn new(text: &'a str, width: u32, style: U8g2TextStyle<Rgb565>) -> Self {
        let text_width = |line: &str| {
            style
                .measure_string(line, Point::zero(), Baseline::Top)
                .next_position
                .x
        };

        let mut first = text;
        while text_width(first) > width as i32 {
            let Some((prefix, _)) = first.rsplit_once(' ') else {
                break;
            };
            first = prefix;
        }

        let remainder = text[first.len()..].trim_start();
        let second = (!remainder.is_empty()).then_some(remainder);

        Self {
            first,
            second,
            style,
            width,
            position: Point::zero(),
        }
    }

    pub fn height(&self) -> u32 {
        (1 + u32::from(self.second.is_some())) * self.style.line_height()
    }

    pub fn at(mut self, position: Point) -> Self {
        self.position = position;
        self
    }
}

impl Drawable for TextBlock<'_> {
    type Color = Rgb565;
    type Output = ();

    fn draw<D>(&self, display: &mut D) -> Result<(), D::Error>
    where
        D: DrawTarget<Color = Rgb565>,
    {
        let area = Rectangle::new(self.position, Size::new(self.width, self.height()));
        let mut clipped = display.clipped(&area);

        Text::with_baseline(self.first, self.position, &self.style, Baseline::Top)
            .draw(&mut clipped)?;

        if let Some(second) = self.second {
            Text::with_baseline(
                second,
                self.position + Point::new(0, self.style.line_height() as i32),
                &self.style,
                Baseline::Top,
            )
            .draw(&mut clipped)?;
        }

        Ok(())
    }
}
