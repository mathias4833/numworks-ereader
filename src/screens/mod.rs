use crate::eadk::event::Event;
use crate::reading::ReadingState;
use crate::ui::components::battery::BatteryState;
use book_format::Library;
use embedded_graphics::pixelcolor::Rgb565;
use embedded_graphics::prelude::DrawTarget;

pub mod home;
pub mod library;
pub mod reader;

pub enum ActiveScreen {
    Home(home::HomeScreen),
    Library(library::LibraryScreen),
    Reader(reader::ReaderScreen),
}

pub enum Route {
    Home,
    Library,
    Reader,
}

pub enum ScreenResult {
    None,
    Redraw,
    Navigate(Route),
}

impl ScreenResult {
    pub const fn redraw_if(changed: bool) -> Self {
        if changed { Self::Redraw } else { Self::None }
    }
}

pub struct UpdateContext<'a> {
    pub library: &'a Library<'static>,
    pub reading: &'a mut ReadingState,
}

pub struct ScreenContext<'a> {
    pub battery: &'a BatteryState,
    pub reading: &'a ReadingState,
    pub library: &'a Library<'static>,
}

pub trait Screen {
    fn draw<D>(&self, display: &mut D, ctx: ScreenContext<'_>) -> Result<(), D::Error>
    where
        D: DrawTarget<Color = Rgb565>;

    fn on_event(&mut self, event: Event, ctx: &mut UpdateContext<'_>) -> ScreenResult;
}

impl Screen for ActiveScreen {
    fn draw<D>(&self, display: &mut D, ctx: ScreenContext<'_>) -> Result<(), D::Error>
    where
        D: DrawTarget<Color = Rgb565>,
    {
        match self {
            Self::Home(screen) => screen.draw(display, ctx),
            Self::Library(screen) => screen.draw(display, ctx),
            Self::Reader(screen) => screen.draw(display, ctx),
        }
    }

    fn on_event(&mut self, event: Event, ctx: &mut UpdateContext<'_>) -> ScreenResult {
        match self {
            Self::Home(screen) => screen.on_event(event, ctx),
            Self::Library(screen) => screen.on_event(event, ctx),
            Self::Reader(screen) => screen.on_event(event, ctx),
        }
    }
}
