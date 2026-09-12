use crate::eadk;
use crate::eadk::event::Event;
use crate::reading::ReadingState;
use crate::screens::home::HomeScreen;
use crate::screens::reader::ReaderScreen;
use crate::screens::{DrawContext, Screen};
use crate::ui::components::battery::BatteryState;
use crate::ui::display::EadkDisplay;

const EVENT_TIMEOUT_MS: i32 = 1000;

pub enum AppScreen {
    Home(HomeScreen),
    Reader(ReaderScreen),
}

pub enum AppAction {
    None,
    Redraw,
    OpenReader,
    GoHome,
    PreviousPage,
    NextPage,
}

impl AppAction {
    pub const fn redraw_if(cond: bool) -> Self {
        if cond { Self::Redraw } else { Self::None }
    }
}

pub struct App {
    display: EadkDisplay,
    screen: AppScreen,
    battery: BatteryState,
    reading: ReadingState,
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}

impl App {
    pub fn new() -> Self {
        Self {
            display: EadkDisplay::new(),
            screen: AppScreen::Home(HomeScreen::new()),
            battery: BatteryState::new(),
            reading: ReadingState::new(crate::book::load().unwrap()),
        }
    }

    pub fn run(&mut self) -> ! {
        self.battery
            .update_if_needed(eadk::timing::millis(), Event::None);
        self.draw();

        loop {
            let event = eadk::event::wait_event(EVENT_TIMEOUT_MS);
            let now = eadk::timing::millis();

            let battery_changed = self.battery.update_if_needed(now, event);

            let action = match event {
                Event::None | Event::Idle => AppAction::None,
                Event::BatteryCharging | Event::USBPlug => AppAction::Redraw,
                _ => self.current_screen_handle_event(event),
            };
            let needs_redraw = self.handle_action(action);

            if battery_changed || needs_redraw {
                self.draw();
            }
        }
    }

    fn current_screen_handle_event(&mut self, event: Event) -> AppAction {
        match &mut self.screen {
            AppScreen::Home(screen) => screen.handle_event(event),
            AppScreen::Reader(screen) => screen.handle_event(event),
        }
    }

    /// Returns `true` if the screen needs to be redrawn.
    fn handle_action(&mut self, action: AppAction) -> bool {
        match action {
            AppAction::None => false,

            AppAction::Redraw => true,

            AppAction::OpenReader => {
                self.screen = AppScreen::Reader(ReaderScreen::new());
                true
            }

            AppAction::GoHome => {
                self.screen = AppScreen::Home(HomeScreen::new());
                true
            }

            AppAction::PreviousPage => self.reading.previous_page(),

            AppAction::NextPage => self.reading.next_page(),
        }
    }

    fn draw(&mut self) {
        let ctx = DrawContext {
            battery: &self.battery,
            reading: &self.reading,
        };

        let _ = match &self.screen {
            AppScreen::Home(screen) => screen.draw(&mut self.display, ctx),
            AppScreen::Reader(screen) => screen.draw(&mut self.display, ctx),
        };
    }
}
