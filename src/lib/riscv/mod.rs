pub enum PrivilegeMode {
    User,
    Supervisor,
    Machine,
}

impl PrivilegeMode {
    pub fn code(&self) -> u64 {
        match self {
            PrivilegeMode::User => 0b00,
            PrivilegeMode::Supervisor => 0b01,
            PrivilegeMode::Machine => 0b11,
        }
    }
}

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
