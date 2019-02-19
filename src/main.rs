#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points

mod panic_handler;
mod vga_buffer;

#[no_mangle]
pub extern "C" fn _start() {
    println!("Hello World{}", "!");
    loop {}
}
