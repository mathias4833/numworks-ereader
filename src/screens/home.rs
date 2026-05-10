use crate::app::AppAction;
use crate::eadk::display::Color;
use crate::eadk::event::Event;
use crate::screens::{DrawContext, Screen};
use crate::ui;
use crate::ui::layout::{Frame, Insets, Stack, SCREEN};
use crate::ui::widgets::menu::Menu;
use crate::ui::widgets::status_bar::StatusBar;
use core::ffi::CStr;

static HOME_ITEMS: [&CStr; 2] = [c"Open reader", c"Settings"];

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
    fn draw(&self, ctx: DrawContext<'_>) {
        let frame = Frame::new(SCREEN)
            .with_status_bar(24)
            .with_bottom_bar(24)
            .with_padding(Insets::all(4));

        ui::draw::fill(Color::WHITE);

        StatusBar::new(c"Numworks Reader")
            .with_battery(ctx.battery)
            .draw(frame.status_bar());

        self.menu.draw(Stack::vertical(frame.content(), 45, 4));
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
