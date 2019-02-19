use core::panic::PanicInfo;

/// Panic handler for halting when `panic` is called.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    crate::println!("{}", info);
    loop {}
}
