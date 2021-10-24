#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    // This function never returns, therefore it's marked with !
    // see diverging function
    loop { }
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    loop { }
}
