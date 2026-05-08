#![no_std]
#![no_main]

use numworks_ereader::app_main;

#[unsafe(no_mangle)]
pub extern "C" fn main() -> ! {
    app_main()
}
