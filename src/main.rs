#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

/// Overwrite the Entry Point, cuz there is no underlying OS.
/// Hardware must be initialized manually.
#[no_mangle]
pub extern "C" fn _start() -> ! {
    loop {}
}
