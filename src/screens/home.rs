use crate::app::AppAction;
use crate::eadk::event::Event;
use crate::screens::{DrawContext, Screen};
use crate::ui::components::menu::Menu;
use crate::ui::components::status_bar::StatusBar;
use crate::ui::layout::{Frame, Insets, Stack, SCREEN};
use embedded_graphics::pixelcolor::Rgb565;
use embedded_graphics::prelude::*;
use embedded_graphics::primitives::PrimitiveStyle;

static HOME_ITEMS: [&str; 2] = ["Open reader", "Settings"];

pub struct HomeScreen {
    menu: Menu<'static>,
}

impl HomeScreen {
    pub const fn new() -> Self {
        Self {
            menu: Menu::new(&HOME_ITEMS),
        }
    }
}

impl Screen for HomeScreen {
    fn draw<D>(&self, display: &mut D, ctx: DrawContext<'_>) -> Result<(), D::Error>
    where
        D: DrawTarget<Color = Rgb565>,
    {
        let frame = Frame::new(SCREEN)
            .with_status_bar(24)
            .with_bottom_bar(24)
            .with_padding(Insets::all(4));

        SCREEN
            .into_styled(PrimitiveStyle::with_fill(Rgb565::WHITE))
            .draw(display)?;

        StatusBar::new(frame.status_bar(), "Numworks Reader")
            .with_battery(ctx.battery)
            .draw(display)?;

        self.menu
            .view(Stack::vertical(frame.content(), 45, 4))
            .draw(display)?;

        Ok(())
    }

    fn handle_event(&mut self, event: Event) -> AppAction {
        match event {
            Event::Up => AppAction::redraw_if(self.menu.move_up()),

            Event::Down => AppAction::redraw_if(self.menu.move_down()),

            Event::Ok => match self.menu.selected() {
                0 => AppAction::OpenReader,
                1 => AppAction::None, // TODO: Settings
                _ => AppAction::None,
            },

            _ => AppAction::None,
        }
    }
}
