use crate::eadk;
use crate::eadk::event::Event;
use crate::reading::ReadingState;
use crate::screens::home::HomeScreen;
use crate::screens::library::LibraryScreen;
use crate::screens::reader::ReaderScreen;
use crate::screens::{ActiveScreen, Route, Screen, ScreenContext, ScreenResult, UpdateContext};
use crate::ui::components::battery::BatteryState;
use crate::ui::display::EadkDisplay;
use crate::ui::invalidation::Invalidation;
use crate::ui::render_buffer::RenderBuffer;
use book_format::Library;

const EVENT_TIMEOUT_MS: i32 = 1000;

pub struct App {
    display: EadkDisplay,
    render_buffer: RenderBuffer,
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
            render_buffer: RenderBuffer::new(),
            screen: ActiveScreen::Home(HomeScreen::new()),
            library,
            battery: BatteryState::new(),
            reading,
        }
    }

    pub fn run(&mut self) -> ! {
        self.battery
            .update_if_needed(eadk::timing::millis(), Event::None);
        self.draw(Invalidation::Full);

        loop {
            let event = eadk::event::wait_event(EVENT_TIMEOUT_MS);
            let now = eadk::timing::millis();
            let mut invalidation = Invalidation::None;

            if self.battery.update_if_needed(now, event) {
                invalidation.full();
            }

            let result = match event {
                Event::None | Event::Idle | Event::BatteryCharging | Event::USBPlug => {
                    ScreenResult::None
                }
                _ => {
                    let mut ctx = UpdateContext {
                        library: &self.library,
                        reading: &mut self.reading,
                        invalidation: &mut invalidation,
                    };
                    self.screen.on_event(event, &mut ctx)
                }
            };

            if let ScreenResult::Navigate(route) = result {
                self.navigate(route);
                invalidation.full();
            }

            self.draw(invalidation);
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

    fn draw(&mut self, invalidation: Invalidation) {
        if matches!(invalidation, Invalidation::None) {
            return;
        }

        let ctx = ScreenContext {
            battery: &self.battery,
            reading: &self.reading,
            library: &self.library,
        };

        match invalidation {
            Invalidation::None => {}

            Invalidation::Full => {
                // The display refreshes at 40 Hz (~25 ms/frame). Since drawing is done
                // directly to the screen, wait for VBlank to reduce visible tearing.
                // See: https://github.com/numworks/epsilon/issues/2401
                eadk::display::wait_for_vblank();
                let _ = self.screen.draw(&mut self.display, ctx);
            }

            Invalidation::Partial(area) => {
                if self.render_buffer.can_hold(area) {
                    self.render_buffer.set_area(area);
                    let _ = self.screen.draw(&mut self.render_buffer, ctx);

                    eadk::display::wait_for_vblank();
                    eadk::display::push_rect(area, self.render_buffer.pixels());
                } else {
                    eadk::display::wait_for_vblank();
                    let _ = self.screen.draw(&mut self.display, ctx);
                }
            }
        }
    }
}
