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
extern "C" fn rust_begin_panic() -> ! {
    Console.lock().set_cursor(0, 0);
    Console.lock().set_color(Color::new(HalfColor::LightRed, HalfColor::Black));
    println!("*** KERNEL PANIC");
    loop {}
}

#[no_mangle]
pub extern "C" fn kmain_setup(multiboot2_addr: usize) {
    Console.lock().clear_screen();
    let boot_info = unsafe { multiboot2::load(multiboot2_addr) };
    let memory_map = boot_info.memory_map_tag().unwrap();
    println!("Memory areas:");
    for area in memory_map.memory_areas() {
        println!("\tstart: 0x{:x}, length: 0x{:x}",
                 area.base_addr,
                 area.length);
    }
}

#[no_mangle]
pub extern "C" fn kmain() {
    println!("Hello from Rite!");
}
