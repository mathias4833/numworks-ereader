use crate::eadk;
use crate::eadk::event::Event;
use crate::reading::ReadingState;
use crate::screens::home::HomeScreen;
use crate::screens::library::LibraryScreen;
use crate::screens::reader::ReaderScreen;
use crate::screens::{Screen, ScreenContext};
use crate::ui::components::battery::BatteryState;
use crate::ui::display::EadkDisplay;
use book_format::Library;

const EVENT_TIMEOUT_MS: i32 = 1000;

pub enum AppScreen {
    Home(HomeScreen),
    Reader(ReaderScreen),
    Library(LibraryScreen),
}

pub enum AppAction {
    None,
    Redraw,
    OpenReader,
    OpenLibrary,
    OpenBook(usize),
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
    library: Library<'static>,
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
        let library = Library::parse(eadk::external_data::get()).unwrap();
        let reading = ReadingState::new(library.book_count())
            .expect("library is empty or contains too many books");

        Self {
            display: EadkDisplay::new(),
            screen: AppScreen::Home(HomeScreen::new()),
            library,
            battery: BatteryState::new(),
            reading,
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
            AppScreen::Library(screen) => screen.handle_event(event),
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

            AppAction::OpenLibrary => {
                self.screen = AppScreen::Library(LibraryScreen::new(
                    &self.library,
                    self.reading.current_book(),
                ));
                true
            }

            AppAction::OpenBook(index) => {
                if !self.reading.select_book(index) {
                    return false;
                }

                self.screen = AppScreen::Reader(ReaderScreen::new());
                true
            }

            AppAction::GoHome => {
                self.screen = AppScreen::Home(HomeScreen::new());
                true
            }

            AppAction::PreviousPage => self.reading.previous_page(),

            AppAction::NextPage => {
                let Ok(Some(book)) = self.library.book(self.reading.current_book()) else {
                    return false;
                };

                self.reading.next_page(book.page_count())
            }
        }
    }

    fn draw(&mut self) {
        let ctx = ScreenContext {
            battery: &self.battery,
            reading: &self.reading,
            library: &self.library,
        };

        let _ = match &self.screen {
            AppScreen::Home(screen) => screen.draw(&mut self.display, ctx),
            AppScreen::Library(screen) => screen.draw(&mut self.display, ctx),
            AppScreen::Reader(screen) => screen.draw(&mut self.display, ctx),
        };
    }
}
