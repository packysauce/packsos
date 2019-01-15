#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]
#![cfg_attr(test, allow(unused_imports))]

use acpi::search_for_rsdp_bios;
use core::panic::PanicInfo;
use packsos::{println, serial_print, serial_println};
use x86_64::instructions::interrupts;

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    packsos::halt();
}

#[cfg(not(test))]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    packsos::gdt::init();
    packsos::interrupts::init_idt();
    unsafe { packsos::interrupts::PIC.lock().initialize() };
    interrupts::enable();

    serial_println!("Smash circuits online");
    println!("Lemme smash");
    println!("You want some blue?");
    packsos::halt();
}
