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
use ross::task::executor::Executor;
use ross::task::keyboard;
use ross::task::{simple_executor::SimpleExecutor, Task};
use ross::{init, println};

entry_point!(kernel_main);

async fn async_number() -> u32 {
    40
}

async fn example_task() {
    let number = async_number().await;
    println!("async number: {}", number);
}

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    init(boot_info);

    let mut exectutor = Executor::new();
    exectutor.spawn(Task::new(example_task()));
    exectutor.spawn(Task::new(keyboard::print_keypresses()));
    exectutor.run();

    #[cfg(test)]
    test_main();
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
