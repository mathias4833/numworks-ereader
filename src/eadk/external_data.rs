#[cfg(target_os = "none")]
unsafe extern "C" {
    static eadk_external_data: *const u8;
    static eadk_external_data_size: usize;
}

#[cfg(target_os = "none")]
pub fn get() -> &'static [u8] {
    unsafe { core::slice::from_raw_parts(eadk_external_data, eadk_external_data_size) }
}

#[cfg(not(target_os = "none"))]
pub fn get() -> &'static [u8] {
    include_bytes!("../../assets/library.nwlib")
}
