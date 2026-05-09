mod insets;
mod stack;

use crate::eadk::display::{Rect, DISPLAY_HEIGHT, DISPLAY_WIDTH};
pub use insets::Insets;
pub use stack::Stack;

pub const SCREEN: Rect = Rect::new(0, 0, DISPLAY_WIDTH, DISPLAY_HEIGHT);
