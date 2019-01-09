#![no_std]
#![feature(abi_x86_interrupt)]
#![cfg_attr(not(test), no_main)]
#![cfg_attr(test, allow(dead_code, unused_macros, unused_imports))]

use core::panic::PanicInfo;
use core::sync::atomic::{AtomicUsize, Ordering};
use lazy_static::lazy_static;
use packsos::{exit_qemu, serial_println};
use x86_64::structures::idt::{InterruptDescriptorTable, ExceptionStackFrame};

static HANDLER_CALLED: AtomicUsize = AtomicUsize::new(0);

#[no_mangle]
pub extern "C" fn _start() -> ! {
    init_test_idt();

    x86_64::instructions::int3();

    match HANDLER_CALLED.load(Ordering::SeqCst) {
        1 => serial_println!("ok"),
        0 => {
            serial_println!("failed");
            serial_println!("Breakpoint handler not called");
        },
        other => {
            serial_println!("failed");
            serial_println!("Breakpoint handler called {} times", other);
        }
    }

    unsafe { exit_qemu() };
    loop {}
}

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(bp_handler);
        idt
    };
}

pub fn init_test_idt() {
    IDT.load();
}

extern "x86-interrupt" fn bp_handler(
    stack: &mut ExceptionStackFrame)
{
    HANDLER_CALLED.fetch_add(1, Ordering::SeqCst);
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    serial_println!("failed");
    serial_println!("{}", info);

    unsafe {
        exit_qemu();
    }

    loop {}
}
