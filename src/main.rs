#![no_std]
#![no_main]
#![feature(asm)]
#![feature(const_mut_refs)]
#![feature(custom_test_frameworks)]
#![feature(type_alias_impl_trait)]
#![test_runner(ross::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;
use ross::{init, println};

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    init(boot_info);

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
