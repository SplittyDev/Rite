#![feature(lang_items)]
#![feature(const_fn)]
#![feature(unique)]
#![no_std]

extern crate rlibc;
extern crate spin;
extern crate cpuio;
extern crate multiboot2;

use core::fmt::Write;

#[macro_use]
mod vga;
use vga::{Console, Color, HalfColor};
mod serial;
use serial::COM1;
mod memory;
use memory::FrameAllocator;

#[lang = "eh_personality"]
extern "C" fn eh_personality() {}

#[lang = "panic_fmt"]
extern "C" fn panic_fmt(fmt: core::fmt::Arguments, file: &str, line: u32) -> ! {
    Console.lock().set_cursor(0, 0);
    Console.lock().set_color(Color::new(HalfColor::LightRed, HalfColor::Black));
    println!("***\tKERNEL PANIC\n\tin {} at line {}:\n\t{}",
             file,
             line,
             fmt);
    loop {}
}

/// Early kernel entry point.
#[no_mangle]
pub extern "C" fn kmain_setup(multiboot2_addr: usize) {

    // Clear the VGA buffer
    Console.lock().clear_screen();

    // Print multiboot2 debug information
    debug_print_multiboot2_info(multiboot2_addr);

    // Initialize COM1
    COM1.lock().init();

    // Test the serial writer
    write!(COM1.lock(), "Hello, world!");
}

/// Main kernel entry point.
#[no_mangle]
pub extern "C" fn kmain() -> ! {
    println!("Hello from Rite!");
    loop {}
}

fn debug_print_multiboot2_info(multiboot2_addr: usize) {
    // Get the multiboot2 data
    let mb2_info = unsafe { multiboot2::load(multiboot2_addr) };

    // Get the memory mapping and print the memory areas
    let memory_map = mb2_info.memory_map_tag().unwrap();
    println!("Memory areas:");
    for area in memory_map.memory_areas() {
        println!("\tStart: 0x{:x}; Length: 0x{:x}",
                 area.base_addr,
                 area.length);
    }

    // Get the elf sections and print them
    let elf_sections = mb2_info.elf_sections_tag().unwrap();
    println!("Kernel sections:");
    for section in elf_sections.sections() {
        println!("\tAddr: 0x{:x}; Size: 0x{:x}; Flags: 0x{:x}",
                 section.addr,
                 section.size,
                 section.flags);
    }

    // Get the kernel and multiboo2 memory bounds and print them
    let kernel_start = elf_sections.sections().map(|s| s.addr).min().unwrap();
    let kernel_end = elf_sections.sections().map(|s| s.addr + s.size).max().unwrap();
    let mb2_start = multiboot2_addr;
    let mb2_end = mb2_start + (mb2_info.total_size as usize);
    println!("Kernel start: 0x{:x}\nKernel end: 0x{:x}",
             kernel_start,
             kernel_end);
    println!("Multiboot2 start: 0x{:x}\nMultiboot2 end: 0x{:x}",
             mb2_start,
             mb2_end);
}
