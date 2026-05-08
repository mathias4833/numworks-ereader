#![no_std]
#![no_main]

use numworks_ereader::app::App;

#[unsafe(no_mangle)]
pub extern "C" fn main() -> ! {
    App::new().run()
}
