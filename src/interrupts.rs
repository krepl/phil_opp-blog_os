#![cfg(not(windows))] // don't compile unit tests for Windows target
                      // NOTE: we still compile for the x86_64-blog_os.json target

use crate::println;
use x86_64::structures::idt::{ExceptionStackFrame, InterruptDescriptorTable};

use lazy_static::lazy_static;

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        idt
    };
}

pub fn init_idt() {
    IDT.load();
}

extern "x86-interrupt" fn breakpoint_handler(stack_frame: &mut ExceptionStackFrame) {
    println!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
}
