#[macro_export]
macro_rules! mret {
    () => {
        unsafe {
            core::arch::asm!("mret");
        };
    };
}

#[macro_export]
macro_rules! sret {
    () => {
        unsafe {
            core::arch::asm!("sret");
        };
    };
}

#[macro_export]
macro_rules! ecall {
    () => {
        unsafe {
            core::arch::asm!("ecall");
        };
    };
}

#[macro_export]
macro_rules! wfi {
    () => {
        unsafe {
            core::arch::asm!("wfi");
        };
    };
}
