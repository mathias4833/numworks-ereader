use crate::app::AppAction;
use crate::eadk::display::{Color, Rect};
use crate::eadk::event::Event;
use crate::screens::Screen;
use crate::ui;
use crate::ui::layout::{Insets, Stack, SCREEN};
use crate::ui::widgets::menu::Menu;
use core::ffi::CStr;

static HOME_ITEMS: [&CStr; 2] = [c"Open reader", c"Settings"];
pub const CONTENT: Rect = Insets::new(16, 48, 16, 16).apply(SCREEN);
const HOME_MENU_LAYOUT: Stack = Stack::vertical(CONTENT, 26, 4);

pub struct HomeScreen {
    menu: Menu<'static>,
}

impl HomeScreen {
    pub const fn new() -> Self {
        Self {
            menu: Menu::new(&HOME_ITEMS, HOME_MENU_LAYOUT),
        }
    }
}

impl Screen for HomeScreen {
    fn draw(&self) {
        ui::draw::fill(Color::WHITE);
        ui::draw::title(c"NumWorks Reader");

        self.menu.draw();
    }

    fn handle_event(&mut self, event: Event) -> AppAction {
        match event {
            Event::Up => {
                self.menu.move_up();
                AppAction::Redraw
            }

            Event::Down => {
                self.menu.move_down();
                AppAction::Redraw
            }

            Event::Ok => match self.menu.selected() {
                0 => AppAction::OpenReader,
                1 => AppAction::None, // TODO: Settings
                _ => AppAction::None,
            },

            _ => AppAction::None,
        }
    }
}
