use crate::eadk;
use crate::eadk::event::Event;
use crate::screens::home::HomeScreen;
use crate::screens::reader::ReaderScreen;
use crate::screens::Screen;

pub enum AppScreen {
    Home(HomeScreen),
    Reader(ReaderScreen),
}

pub enum AppAction {
    None,
    Redraw,
    OpenReader,
    GoHome,
}

pub struct App {
    screen: AppScreen,
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}

impl App {
    pub fn new() -> Self {
        Self {
            screen: AppScreen::Home(HomeScreen::new()),
        }
    }

    pub fn run(&mut self) -> ! {
        self.draw();

        loop {
            let event = eadk::event::wait_event();

            let action = self.current_screen_handle_event(event);
            self.handle_action(action);
        }
    }

    fn current_screen_handle_event(&mut self, event: Event) -> AppAction {
        match &mut self.screen {
            AppScreen::Home(screen) => screen.handle_event(event),
            AppScreen::Reader(screen) => screen.handle_event(event),
        }
    }

    fn handle_action(&mut self, action: AppAction) {
        match action {
            AppAction::None => {}

            AppAction::Redraw => {
                self.draw();
            }

            AppAction::OpenReader => {
                self.screen = AppScreen::Reader(ReaderScreen::new());
                self.draw();
            }

            AppAction::GoHome => {
                self.screen = AppScreen::Home(HomeScreen::new());
                self.draw();
            }
        }
    }

    fn draw(&self) {
        match &self.screen {
            AppScreen::Home(screen) => screen.draw(),
            AppScreen::Reader(screen) => screen.draw(),
        }
    }
}
