use crate::eadk;
use crate::eadk::event::Event;
use crate::reading::ReadingState;
use crate::screens::home::HomeScreen;
use crate::screens::library::LibraryScreen;
use crate::screens::reader::ReaderScreen;
use crate::screens::{ActiveScreen, Route, Screen, ScreenContext, ScreenResult, UpdateContext};
use crate::ui::components::battery::BatteryState;
use crate::ui::display::EadkDisplay;
use book_format::Library;

const EVENT_TIMEOUT_MS: i32 = 1000;

pub struct App {
    display: EadkDisplay,
    screen: ActiveScreen,
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
            screen: ActiveScreen::Home(HomeScreen::new()),
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
            let system_redraw = matches!(event, Event::BatteryCharging | Event::USBPlug);

            let result = match event {
                Event::None | Event::Idle | Event::BatteryCharging | Event::USBPlug => {
                    ScreenResult::None
                }
                _ => {
                    let mut ctx = UpdateContext {
                        library: &self.library,
                        reading: &mut self.reading,
                    };
                    self.screen.on_event(event, &mut ctx)
                }
            };
            let needs_redraw = match result {
                ScreenResult::None => false,
                ScreenResult::Redraw => true,
                ScreenResult::Navigate(route) => {
                    self.navigate(route);
                    true
                }
            };

            if system_redraw || battery_changed || needs_redraw {
                self.draw();
            }
        }
    }

    fn navigate(&mut self, route: Route) {
        self.screen = match route {
            Route::Reader => ActiveScreen::Reader(ReaderScreen::new()),
            Route::Library => ActiveScreen::Library(LibraryScreen::new(
                &self.library,
                self.reading.current_book(),
            )),
            Route::Home => ActiveScreen::Home(HomeScreen::new()),
        };
    }

    fn draw(&mut self) {
        // The display refreshes at 40 Hz (~25 ms/frame). Since drawing is done
        // directly to the screen, wait for VBlank to reduce visible tearing.
        // See: https://github.com/numworks/epsilon/issues/2401
        eadk::display::wait_for_vblank();

        let ctx = ScreenContext {
            battery: &self.battery,
            reading: &self.reading,
            library: &self.library,
        };

        let _ = self.screen.draw(&mut self.display, ctx);
    }
}
