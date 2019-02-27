#![cfg_attr(not(test), no_std)] // don't link the Rust standard library
#![cfg_attr(not(test), no_main)] // disable all Rust-level entry points, except for unit-tests

#[cfg(not(test))]
mod panic_handler;

use blog_os::println;

// NOTE: the cross-platform LLD linker looks for an entry point named `_start`
#[cfg(not(test))] // avoid naming conflict (with `_start`) when running unit-tests
#[no_mangle]
pub extern "C" fn _start() {
    println!("Hello World{}", "!");

    blog_os::interrupts::init_idt();

    x86_64::instructions::int3();

    println!("It did not crash!");
    loop {}
}
