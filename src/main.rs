#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]
#![cfg_attr(test, allow(unused_imports))]

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
    let page_table = 0xffff_ffff_ffff_f000 as *const u64;
    interrupts::enable();

    serial_println!("Smash circuits online");
    for i in 0..512 {
        let entry = unsafe { *page_table.offset(i) };
        if entry != 0 {
            serial_println!("Entry {:3}: {:#x}", i, entry);
        }
    }
    println!("Lemme smash");
    println!("You want some blue?");
    packsos::halt();
}
