// Battery support is split between device and simulator
//
// The EADK battery API are declared in eadk.h, but are not provided on the real hardware.
// See: https://github.com/numworks/epsilon/issues/2326
//
// To bypass that, on the device we use the same SVC calls as Epsilon internally

use num_enum::{IntoPrimitive, TryFromPrimitive};

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, IntoPrimitive, TryFromPrimitive)]
pub enum BatteryLevel {
    Empty = 0,
    Low = 1,
    Percent40 = 2,
    Percent60 = 3,
    Percent80 = 4,
    Full = 5,
}

#[cfg(target_os = "none")]
pub fn is_charging() -> bool {
    let value: u32;

    unsafe {
        core::arch::asm!(
            "svc #3",
            lateout("r0") value,
            lateout("r1") _,
            lateout("r2") _,
            lateout("r3") _,
            options(nostack),
        );
    }

    value != 0
}

#[cfg(target_os = "none")]
pub fn level() -> Option<BatteryLevel> {
    let value: u32;

    unsafe {
        core::arch::asm!(
        "svc #4",
        lateout("r0") value,
        lateout("r1") _,
        lateout("r2") _,
        lateout("r3") _,
        options(nostack),
        );
    }

    BatteryLevel::try_from(value as u8).ok()
}

#[cfg(not(target_os = "none"))]
pub fn is_charging() -> bool {
    unsafe { eadk_battery_is_charging() }
}

#[cfg(not(target_os = "none"))]
pub fn level() -> Option<BatteryLevel> {
    let value = unsafe { eadk_battery_level() };
    BatteryLevel::try_from(value).ok()
}

#[cfg(not(target_os = "none"))]
unsafe extern "C" {
    fn eadk_battery_is_charging() -> bool;
    fn eadk_battery_level() -> u8;
}
