#![cfg_attr(target_os = "none", no_std)]

pub mod app;
pub mod eadk;
mod metadata;

#[cfg(target_os = "none")]
mod panic;

// For the emulator
#[cfg(not(target_os = "none"))]
#[unsafe(no_mangle)]
pub extern "C" fn main(_argc: i32, _argv: *const *const u8) -> i32 {
    app::run()
}
