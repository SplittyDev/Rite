#![feature(lang_items)]
#![feature(const_fn)]
#![feature(unique)]
#![no_std]

extern crate rlibc;
extern crate spin;

#[macro_use]
mod vga;

#[lang = "eh_personality"]
extern "C" fn eh_personality() {}

#[lang = "panic_fmt"]
extern "C" fn rust_begin_panic() -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn kmain() -> ! {
    vga::Console.lock().clear_screen();
    print!("\n");
    print!("=> Hello from Rite!");
    loop {}
}
