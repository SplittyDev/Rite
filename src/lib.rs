#![feature(lang_items)]
#![feature(const_fn)]
#![feature(unique)]
#![no_std]

extern crate rlibc;
extern crate spin;

// TODO: Replace this with a proper multiboot2 module
extern crate multiboot2;

#[macro_use]
mod vga;
use vga::{Console, Color, HalfColor};

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

#[no_mangle]
pub extern "C" fn kmain_setup(multiboot2_addr: usize) {
    Console.lock().clear_screen();

    let boot_info = unsafe { multiboot2::load(multiboot2_addr) };

    let memory_map = boot_info.memory_map_tag().unwrap();
    println!("Memory areas:");
    for area in memory_map.memory_areas() {
        println!("\tStart: 0x{:x}; Length: 0x{:x}",
                 area.base_addr,
                 area.length);
    }

    let elf_sections = boot_info.elf_sections_tag().unwrap();
    println!("Kernel sections:");
    for section in elf_sections.sections() {
        println!("\tAddr: 0x{:x}; Size: 0x{:x}; Flags: 0x{:x}",
                 section.addr,
                 section.size,
                 section.flags);
    }

    let kernel_start = elf_sections.sections().map(|s| s.addr).min().unwrap();
    let kernel_end = elf_sections.sections().map(|s| s.addr + s.size).max().unwrap();
    let mb2_start = multiboot2_addr;
    let mb2_end = mb2_start + (boot_info.total_size as usize);
    println!("Kernel start: 0x{:x}\nKernel end: 0x{:x}",
             kernel_start,
             kernel_end);
    println!("Multiboot2 start: 0x{:x}\nMultiboot2 end: 0x{:x}",
             mb2_start,
             mb2_end);
}

#[no_mangle]
pub extern "C" fn kmain() {
    println!("Hello from Rite!");
}
