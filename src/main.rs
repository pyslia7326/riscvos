#![no_main]
#![no_std]

mod start;

use core::panic::PanicInfo;
use lib::csr;
use lib::plic::plic_init;
use lib::riscv::PrivilegeMode;
use lib::shell;
use lib::timer::timer_init;
use lib::trap::kernel_trap::kernel_trap;
use lib::trap::user_trap::user_trap;
use lib::uart::{print_char, uart_init};

static STARTUP_MESSAGE: &[u8] = include_bytes!("startup_message.txt");

#[unsafe(no_mangle)]
fn main() -> ! {
    for &c in STARTUP_MESSAGE.iter() {
        print_char(c as char);
    }
    // Configure PMP to allow full access to all memory
    csr::write_pmpaddr0(0x3FFFFFFFFFFFFF); // Set PMP address to cover all memory
    csr::write_pmpcfg0(0xF); // Enable R/W/X permissions with NA4 address matching
    // pmpcfg0 breakdown:
    // - Bits 0-2: R=1, W=1, X=1 (Allow read, write, and execute)
    // - Bits 3-4: A=2 (NA4, Naturally Aligned 4-byte region)
    // - Bit 7: L=0 (Not locked, can be modified)
    // Alignment: NA4 means the region is aligned to a 4-byte boundary.

    lib::task::scheduler::init();

    csr::write_mtvec(kernel_trap as u64);
    csr::write_mideleg(lib::trap::interrupt::ENABLE_ALL_INTERRUPTS);
    csr::write_medeleg(lib::trap::exception::ENABLE_ALL_EXCEPTIONS);

    // let kernel_task_struct = scheduler::get_kernel_task_struct();
    // csr::write_mscratch(kernel_task_struct as *const task::TaskStruct as u64);
    csr::mstatus_set_pp(PrivilegeMode::Supervisor);
    csr::write_mepc(kernel as u64);
    timer_init();
    lib::mret!();

    loop {}
}

#[unsafe(no_mangle)]
fn kernel() -> ! {
    lib::task::scheduler::task_create(shell::shell as *const u8, "".as_ptr(), 0);

    csr::write_stvec(user_trap as u64);
    csr::sstatus_set_pp(PrivilegeMode::Supervisor);

    csr::write_sstatus(csr::read_sstatus() | (1 << csr::SSTATUS_SPIE)); // Enable S-mode interrupts after sret (switch to idle_task)
    csr::write_sie(csr::read_sie() | (1 << csr::SIE_SSIE)); // Enable software interrupt
    csr::write_sie(csr::read_sie() | (1 << csr::SIE_SEIE));
    plic_init();
    uart_init();
    lib::sret!();

    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo<'_>) -> ! {
    lib::uart::print_string("panic\n");
    if let Some(location) = info.location() {
        lib::uart::print_string(location.file());
        lib::uart::print_char('\n');
        lib::uart::print_integerln(location.line() as u64);
        lib::uart::print_char('\n');
    }
    if let Some(message) = info.message().as_str() {
        lib::uart::print_string(message);
        lib::uart::print_char('\n');
    }
    // Infinite loop on panic
    loop {}
}
