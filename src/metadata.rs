#[used]
#[unsafe(no_mangle)]
#[unsafe(link_section = ".rodata.eadk_app_name")]
pub static EADK_APP_NAME: [u8; 16] = *b"NumWorks Reader\0";

#[used]
#[unsafe(no_mangle)]
#[unsafe(link_section = ".rodata.eadk_api_level")]
pub static EADK_API_LEVEL: u32 = 0;

#[used]
#[unsafe(no_mangle)]
#[unsafe(link_section = ".rodata.eadk_app_icon")]
pub static EADK_APP_ICON: [u8; 4250] = *include_bytes!("../target/icon.nwi");
