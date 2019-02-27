#![feature(abi_x86_interrupt)]
#![cfg_attr(not(test), no_std)]

#[cfg(test)]
#[macro_use]
extern crate pretty_assertions;

pub mod interrupts;
pub mod serial;
pub mod vga_buffer;

pub unsafe fn exit_qemu() {
    use x86_64::instructions::port::Port;

    let mut port = Port::<u32>::new(0xf4);
    port.write(0);
}
