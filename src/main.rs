#![no_std]
#![no_main]

#[unsafe(no_mangle)]
pub extern "C" fn main() -> ! {
    numworks_ereader::app::run()
}
