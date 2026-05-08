use crate::eadk;
use crate::screens::home::HomeScreen;
use crate::screens::reader::ReaderScreen;
use crate::screens::{Event, Screen};

pub enum AppScreen {
    Home(HomeScreen),
    Reader(ReaderScreen),
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
            let mut timeout = 20;
            eadk::event::get(&mut timeout);

            let event = Event::None; // TODO: Implement

            if self.handle_event(event) {
                self.draw();
            }
        }
    }

    fn handle_event(&mut self, event: Event) -> bool {
        match &mut self.screen {
            AppScreen::Home(screen) => screen.handle_event(event).is_some(),
            AppScreen::Reader(screen) => screen.handle_event(event).is_some(),
        }
    }

    fn draw(&self) {
        match &self.screen {
            AppScreen::Home(screen) => screen.draw(),
            AppScreen::Reader(screen) => screen.draw(),
        }
    }
}
