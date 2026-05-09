use crate::eadk;
use crate::eadk::event::Event;
use crate::screens::home::{HomeAction, HomeScreen};
use crate::screens::reader::ReaderScreen;
use crate::screens::Screen;

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
            if let Some(event) = eadk::event::wait_event()
                && self.handle_event(event)
            {
                self.draw();
            }
        }
    }

    fn handle_event(&mut self, event: Event) -> bool {
        match &mut self.screen {
            AppScreen::Home(screen) => match screen.handle_event(event) {
                Some(HomeAction::OpenReader) => {
                    self.screen = AppScreen::Reader(ReaderScreen::new());
                    true
                }
                None => false,
            },
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
