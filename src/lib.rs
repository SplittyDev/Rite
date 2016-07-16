#![feature(lang_items)]
#![no_std]

#[lang = "eh_personality"]
extern "C" fn eh_personality() {}

#[lang = "panic_fmt"]
extern "C" fn rust_begin_panic() -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn kmain() -> ! {
    unsafe {
        let vga = 0xb8000 as *mut u64;
        *vga = 0x2f592f412f4b2f4f;
    };
    loop {}
}
