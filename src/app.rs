use crate::eadk::{self, Color, Rect};

pub fn run() -> ! {
    eadk::display::fill_screen(Color::WHITE);

    eadk::display::push_rect_uniform(Rect::new(40, 40, 240, 160), Color::RED);

    loop {
        let mut timeout = 20;
        eadk::event::get(&mut timeout);
    }
}
