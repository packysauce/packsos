#![feature(abi_x86_interrupt)]
#![cfg_attr(not(test), no_std)] // don't link the Rust standard library

use bootloader::{entry_point, bootinfo::BootInfo};
use bootloader::bootinfo::MemoryRegionType;
use x86_64::structures::paging::{PageTable, RecursivePageTable};

pub mod gdt;
pub mod interrupts;
pub mod serial;
pub mod vga_buffer;

pub unsafe fn exit_qemu() {
    use x86_64::instructions::port::Port;

    let mut port = Port::<u32>::new(0xf4);
    port.write(0);
}

pub fn halt() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}

#[cfg(not(test))]
pub fn kmain(bootinfo: &'static BootInfo) -> ! {
    gdt::init();
    interrupts::init_idt();
    unsafe { interrupts::PIC.lock().initialize() };
    let pt = 0xffff_ffff_ffff_f000 as *mut PageTable;
    let rpt = RecursivePageTable::new(unsafe { &mut *pt} ).unwrap();

    interrupts::enable();

    for region in bootinfo.memory_map.iter() {
      if region.region_type == MemoryRegionType::Usable {
        serial_println!("{:#?}", region);
      }
    }


    serial_println!("Smash circuits online");
    println!("Lemme smash");
    println!("You want some blue?");
    halt()
}