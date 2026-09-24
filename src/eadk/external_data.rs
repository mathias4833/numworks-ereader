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
    use std::sync::OnceLock;

    static LIBRARY: OnceLock<Vec<u8>> = OnceLock::new();

    LIBRARY
        .get_or_init(|| {
            let path = std::env::var_os("NUMWORKS_EREADER_LIBRARY")
                .expect("simulator library path is missing");
            std::fs::read(path).expect("failed to read simulator library")
        })
        .as_slice()
}
