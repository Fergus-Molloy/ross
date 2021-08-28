#![no_std]
#![no_main]
#![feature(asm)]
#![feature(custom_test_frameworks)]
#![test_runner(ross::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use ross::init;
use ross::println;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    init();

    #[cfg(test)]
    test_main();

    println!("did not crash",);
    ross::hlt_loop();
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    ross::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    ross::test_panic_handler(info)
}
