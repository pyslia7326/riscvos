use core::arch::asm;

#[unsafe(no_mangle)]
fn _start() -> ! {
    unsafe {
        asm!(
            "csrr a0, mhartid",
            "bnez a0, park",
            "la sp, stack_top",
            "j main",
            options(noreturn)
        )
    }
}

#[unsafe(no_mangle)]
fn park() -> ! {
    unsafe {
        loop {
            asm!("wfi");
        }
    }
}
