#![no_main]
#![no_std]

mod start;

use core::panic::PanicInfo;
use lib::uart::{print_string, print_integer};

#[unsafe(no_mangle)]
pub fn main() -> ! {
    print_string("Hello, world!\n");
    print_integer(12345);
    loop {}
}

#[panic_handler]
fn panic(_panic: &PanicInfo<'_>) -> ! {
    loop {}
}
