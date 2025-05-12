#![no_main]
#![no_std]

mod start;

use core::panic::PanicInfo;
use core::ptr;
use lib::csr;
use lib::riscv::PrivilegeMode;
use lib::task;
use lib::task::scheduler;
use lib::timer::timer_init;
use lib::trap::kernel_trap::kernel_trap;
use lib::trap::user_trap::user_trap;
use lib::uart::print_string;

#[unsafe(no_mangle)]
fn main() -> ! {
    print_string("Hello, world!\n");

    // Configure PMP to allow full access to all memory
    csr::write_pmpaddr0(0x3FFFFFFFFFFFFF); // Set PMP address to cover all memory
    csr::write_pmpcfg0(0xF); // Enable R/W/X permissions with NA4 address matching
    // pmpcfg0 breakdown:
    // - Bits 0-2: R=1, W=1, X=1 (Allow read, write, and execute)
    // - Bits 3-4: A=2 (NA4, Naturally Aligned 4-byte region)
    // - Bit 7: L=0 (Not locked, can be modified)
    // Alignment: NA4 means the region is aligned to a 4-byte boundary.

    csr::write_mtvec(kernel_trap as u64);
    csr::write_mideleg(lib::trap::interrupt::ENABLE_ALL_INTERRUPTS);
    csr::write_medeleg(lib::trap::exception::ENABLE_ALL_EXCEPTIONS);

    let kernel_task_struct = scheduler::get_kernel_task_struct();
    csr::write_mscratch(kernel_task_struct as *const task::TaskStruct as u64);
    csr::mstatus_set_pp(PrivilegeMode::Supervisor);
    csr::write_mepc(kernel as u64);
    timer_init();
    lib::mret!();

    loop {}
}

#[unsafe(no_mangle)]
fn kernel() -> ! {
    // Print message and a test integer in S-mode
    print_string("Kernel is running in S-mode!\n");

    scheduler::task_create(task::test_task::user_task1, 1, ptr::null());
    scheduler::task_create(task::test_task::user_task2, 1, ptr::null());
    scheduler::create_idle_task();

    csr::write_stvec(user_trap as u64);
    let idle_task_struct = lib::task::scheduler::get_idle_task_struct();
    csr::write_sepc(idle_task_struct.xepc);
    csr::write_sscratch(idle_task_struct as *const lib::task::TaskStruct as u64);
    csr::sstatus_set_pp(PrivilegeMode::Supervisor);

    csr::write_sstatus(csr::read_sstatus() | (1 << csr::SSTATUS_SPIE)); // Enable S-mode interrupts after sret (switch to idle_task)
    csr::write_sie(csr::read_sie() | (1 << csr::SIE_SSIE)); // Enable software interrupt
    lib::sret!();

    loop {}
}

#[panic_handler]
fn panic(_panic: &PanicInfo<'_>) -> ! {
    // Infinite loop on panic
    loop {}
}
