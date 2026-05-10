mod frame;
mod insets;
mod stack;

use crate::eadk::display::{DISPLAY_HEIGHT, DISPLAY_WIDTH};
use crate::geometry::Rect;
pub use frame::Frame;
pub use insets::Insets;
pub use stack::Stack;

pub const SCREEN: Rect = Rect::new(0, 0, DISPLAY_WIDTH, DISPLAY_HEIGHT);
