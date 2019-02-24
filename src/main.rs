#![cfg_attr(not(test), no_std)] // don't link the Rust standard library
#![cfg_attr(not(test), no_main)] // disable all Rust-level entry points, except for unit-tests

#[cfg(test)]
#[macro_use]
extern crate pretty_assertions;

#[cfg(not(test))]
mod panic_handler;
mod vga_buffer;

#[cfg(not(test))] // avoid naming conflict (with `_start`) when running unit-tests
#[no_mangle]
pub extern "C" fn _start() {
    // NOTE: the cross-platform LLD linker looks for an entry point named `_start`

    println!("Hello World!");
    loop {}
}
