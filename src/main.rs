#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]
#![cfg_attr(test, allow(unused_imports))]

use packsos::{exit_qemu, println, serial_println};
use core::panic::PanicInfo;

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(not(test))]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    packsos::gdt::init();
    packsos::interrupts::init_idt();
    serial_println!("Smash circuits online");

    fn stack_overflow() {
        stack_overflow();
    }

    stack_overflow();

    println!("Lemme smash");
    println!("You want some blue?");
    panic!("no blue");
    loop {}
}
