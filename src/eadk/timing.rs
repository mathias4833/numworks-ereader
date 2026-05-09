// On device, `eadk_timing_millis` is available directly as a `uint64_t eadk_timing_millis()`.
// On the simulator, EADK exposes two 32-bit functions instead.
// See: https://github.com/numworks/epsilon/blob/master/epsilon/eadk/include/eadk/eadk.h

#[cfg(target_os = "none")]
pub fn millis() -> u64 {
    unsafe { eadk_timing_millis() }
}

#[cfg(not(target_os = "none"))]
pub fn millis() -> u64 {
    unsafe {
        let low = _eadk_timing_millis_low() as u64;
        let high = _eadk_timing_millis_high() as u64;

        low | (high << 32)
    }
}

unsafe extern "C" {
    #[cfg(target_os = "none")]
    fn eadk_timing_millis() -> u64;

    #[cfg(not(target_os = "none"))]
    fn _eadk_timing_millis_low() -> u32;

    #[cfg(not(target_os = "none"))]
    fn _eadk_timing_millis_high() -> u32;
}
