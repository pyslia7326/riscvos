#![no_main]
#![no_std]

mod start;

use core::panic::PanicInfo;
use lib::trap::trap_handler;
use lib::uart::{print_integerln, print_string};
use lib::{csr, ecall};

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
    csr::write_mepc(kernel as u64);
    csr::write_mtvec(trap_handler::trap_handler as u64);
    csr::write_mideleg(lib::trap::interrupt::ENABLE_ALL_INTERRUPTS);
    csr::write_medeleg(lib::trap::exception::ENABLE_ALL_EXCEPTIONS);

    // Return to S-mode
    lib::mret!();

    loop {}
}

#[unsafe(no_mangle)]
fn kernel() -> ! {
    // Print message and a test integer in S-mode
    print_string("Kernel is running in S-mode!\n");
    fn user_task1() {
        print_string("User Task 1 is running!\n");
        let mut yield_count = 0;
        loop {
            yield_count += 1;
            print_string("User Task 1 ecall count: ");
            print_integerln(yield_count);
            crate::ecall!();
            // Simulate some work
            let mut i = 0;
            while i < 50000000 {
                i += 1;
            }
        }
    }
    fn user_task2() {
        print_string("User Task 2 is running!\n");
        let mut yield_count = 0;
        loop {
            yield_count += 1;
            print_string("User Task 2 ecall count: ");
            print_integerln(yield_count);
            crate::ecall!();
            // Simulate some work
            let mut i = 0;
            while i < 50000000 {
                i += 1;
            }
        }
    }
    lib::task::scheduler::create_task(user_task1);
    lib::task::scheduler::create_task(user_task2);
    lib::task::scheduler::create_idle_task();
    csr::write_stvec(trap_handler::trap_handler as u64);
    let idle_task_struct = lib::task::scheduler::get_idle_task_struct();
    csr::write_sepc(idle_task_struct.sepc);
    csr::write_sscratch(idle_task_struct as *const lib::task::TaskStruct as u64);
    let mut sstatus = csr::read_sstatus();
    sstatus = (sstatus & !csr::SSTATUS_SPP_MASK) | (0b0 << 8); // Set SPP to U-mode
    csr::write_sstatus(sstatus);
    unsafe {
        // This can be replaced with a single sret instruction
        lib::trap::trap_handler::trap_return();
    }

    loop {}
}

#[panic_handler]
fn panic(_panic: &PanicInfo<'_>) -> ! {
    // Infinite loop on panic
    loop {}
}
