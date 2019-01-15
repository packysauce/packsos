use crate::{gdt, print, println, serial_println};
use lazy_static::lazy_static;
use pic8259_simple::ChainedPics;
use spin;
use x86_64::registers::control::{Cr0, Cr3};
use x86_64::registers::model_specific::Efer;
use x86_64::registers::rflags;
use x86_64::structures::idt::{ExceptionStackFrame, InterruptDescriptorTable};

pub const PIC_1_OFFSET: u8 = 32;
pub const PIC_2_OFFSET: u8 = PIC_1_OFFSET + 8;

pub const TIMER: u8 = PIC_1_OFFSET;
pub const KEYBOARD: u8 = PIC_1_OFFSET + 1;

pub static PIC: spin::Mutex<ChainedPics> =
    spin::Mutex::new(unsafe { ChainedPics::new(PIC_1_OFFSET, PIC_2_OFFSET) });

lazy_static! {
    pub static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        unsafe {
            idt.double_fault
                .set_handler_fn(double_fault_handler)
                .set_stack_index(gdt::DF_IST_INDEX);
        }
        idt[usize::from(TIMER)].set_handler_fn(timer_handler);
        idt[usize::from(KEYBOARD)].set_handler_fn(kbd_handler);
        idt
    };
}

pub fn init_idt() {
    IDT.load();
}

fn dump_registers() {
    serial_println!("CPU Features Active: {:#?}", Efer::read());
    serial_println!("RFLAGS: {:#?}", rflags::read());
    serial_println!("CR0: {:#?}", Cr0::read());
    serial_println!("CR3: {:#?}", Cr3::read());
}

extern "x86-interrupt" fn breakpoint_handler(stack: &mut ExceptionStackFrame) {
    println!("EXCEPTION: BREAKPOINT\n{:#?}", stack);
}

extern "x86-interrupt" fn double_fault_handler(stack: &mut ExceptionStackFrame, _error_code: u64) {
    serial_println!("EXCEPTION: DOUBLE FAULT\n{:#?}", stack);
    dump_registers();

    super::halt();
}

extern "x86-interrupt" fn timer_handler(_stack: &mut ExceptionStackFrame) {
    print!(".");
    unsafe { PIC.lock().notify_end_of_interrupt(TIMER) }
}

extern "x86-interrupt" fn kbd_handler(_stack: &mut ExceptionStackFrame) {
    use pc_keyboard::{layouts, DecodedKey, Keyboard, ScancodeSet1};
    use spin::Mutex;
    use x86_64::instructions::port::Port;

    lazy_static! {
        static ref K: Mutex<Keyboard<layouts::Us104Key, ScancodeSet1>> =
            Mutex::new(Keyboard::new(layouts::Us104Key, ScancodeSet1));
    }

    let mut kbd = K.lock();
    let port = Port::new(0x60);
    let scancode: u8 = unsafe { port.read() };

    if let Ok(Some(event)) = kbd.add_byte(scancode) {
        if let Some(key) = kbd.process_keyevent(event) {
            match key {
                DecodedKey::Unicode(character) => print!("{}", character),
                DecodedKey::RawKey(key) => print!("{:?}", key),
            }
        }
    }

    unsafe { PIC.lock().notify_end_of_interrupt(KEYBOARD) }
}
