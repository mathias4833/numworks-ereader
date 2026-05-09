use crate::eadk::display;
use crate::eadk::display::{Color, Point, Rect, DISPLAY_HEIGHT, DISPLAY_WIDTH};
use core::ffi::CStr;

pub fn fill(color: Color) {
    display::push_rect_uniform(Rect::new(0, 0, DISPLAY_WIDTH, DISPLAY_HEIGHT), color);
}

pub fn text(content: &CStr, x: u16, y: u16, color: Color, background: Color) {
    display::draw_string(content, Point::new(x, y), false, color, background);
}

pub fn large_text(content: &CStr, x: u16, y: u16, color: Color, background: Color) {
    display::draw_string(content, Point::new(x, y), true, color, background);
}

pub fn centered_large_text(content: &CStr, x: u16, y: u16, color: Color, background: Color) {
    let (w, h) = text_size(content, true);
    large_text(
        content,
        x.saturating_sub(w / 2),
        y.saturating_sub(h / 2),
        color,
        background,
    );
}

pub fn centered_text(content: &CStr, x: u16, y: u16, color: Color, background: Color) {
    let (w, h) = text_size(content, false);
    text(
        content,
        x.saturating_sub(w / 2),
        y.saturating_sub(h / 2),
        color,
        background,
    );
}

fn text_size(content: &CStr, large_font: bool) -> (u16, u16) {
    let char_w = if large_font { 10 } else { 7 };
    let char_h = if large_font { 18 } else { 14 };

    let text_w = content.to_bytes().len() as u16 * char_w;
    (text_w, char_h)
}

fn clip_to_screen(area: Rect) -> Option<Rect> {
    let left = area.left().min(DISPLAY_WIDTH);
    let top = area.top().min(DISPLAY_HEIGHT);
    let right = area.right().min(DISPLAY_WIDTH);
    let bottom = area.bottom().min(DISPLAY_HEIGHT);

    if right <= left || bottom <= top {
        return None;
    }

    Some(Rect::new(left, top, right - left, bottom - top))
}

pub fn rect(area: Rect, color: Color) {
    if let Some(area) = clip_to_screen(area) {
        display::push_rect_uniform(area, color);
    }
}

pub fn border(area: Rect, color: Color) {
    if area.width < 1 || area.height < 1 {
        return;
    }

    rect(Rect::new(area.left(), area.top(), area.width, 1), color);

    rect(
        Rect::new(area.left(), area.bottom() - 1, area.width, 1),
        color,
    );
    rect(Rect::new(area.left(), area.top(), 1, area.height), color);
    rect(
        Rect::new(area.right() - 1, area.top(), 1, area.height),
        color,
    );
}
