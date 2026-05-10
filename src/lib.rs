#![cfg_attr(target_os = "none", no_std)]

use crate::app::App;

pub mod app;
pub mod eadk;
pub mod geometry;
mod metadata;
mod screens;
mod ui;

#[cfg(target_os = "none")]
mod panic;

// For the emulator
#[cfg(not(target_os = "none"))]
#[unsafe(no_mangle)]
pub extern "C" fn main(_argc: i32, _argv: *const *const u8) -> i32 {
    App::new().run();
}
