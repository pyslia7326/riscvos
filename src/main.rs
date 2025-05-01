#![no_main]
#![no_std]

mod start;

use core::arch::asm;
use core::panic::PanicInfo;
use lib::csr;
use lib::trap::trap_handler;
use lib::uart::{print_integerln, print_string};

#[unsafe(no_mangle)]
fn main() -> ! {
    print_string("Hello, world!\n");

    // Set MPP (Machine Previous Privilege) to S-mode
    let mut mstatus = csr::read_mstatus();
    mstatus = (mstatus & !csr::MSTATUS_MPP_MASK) | (0b01 << 11);
    csr::write_mstatus(mstatus);

    // Configure PMP to allow full access to all memory
    csr::write_pmpaddr0(0x3FFFFFFFFFFFFF); // Set PMP address to cover all memory
    csr::write_pmpcfg0(0xF); // Enable R/W/X permissions with NA4 address matching
    // pmpcfg0 breakdown:
    // - Bits 0-2: R=1, W=1, X=1 (Allow read, write, and execute)
    // - Bits 3-4: A=2 (NA4, Naturally Aligned 4-byte region)
    // - Bit 7: L=0 (Not locked, can be modified)
    // Alignment: NA4 means the region is aligned to a 4-byte boundary.

    // Set the entry point for S-mode and trap handler
    csr::write_mepc(s_mode_main as u64);
    csr::write_mtvec(trap_handler as u64);

    // Return to S-mode
    unsafe {
        asm!("mret");
    }

    loop {}
}

#[unsafe(no_mangle)]
fn s_mode_main() -> ! {
    // Print message and a test integer in S-mode
    print_string("Now running in S-mode!\n");
    print_integerln(0x12345678);
    loop {}
}

#[panic_handler]
fn panic(_panic: &PanicInfo<'_>) -> ! {
    // Infinite loop on panic
    loop {}
}
