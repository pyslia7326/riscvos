use crate::riscv::PrivilegeMode;

const PLIC_BASE: usize = 0xc000000;
const PLIC_SOURCE_PRIORITY: *mut u32 = (PLIC_BASE + 0x000000) as *mut u32;
const _PLIC_SOURCE_PENDING: *mut u32 = (PLIC_BASE + 0x001000) as *mut u32;
const PLIC_SOURCE_ENABLE: *mut u32 = (PLIC_BASE + 0x002000) as *mut u32;
const PLIC_PRIORITY_THRESHOLD: *mut u32 = (PLIC_BASE + 0x200000) as *mut u32;
const PLIC_CLAIM_COMPLETE: *mut u32 = (PLIC_BASE + 0x200004) as *mut u32;

pub const UART0_IRQ: usize = 10;

macro_rules! plic_source_priority_addr {
    ($source:expr) => {{
        let source_shift = $source * 0x4;
        unsafe { (PLIC_SOURCE_PRIORITY as *mut u8).add(source_shift) as *mut u32 }
    }};
}

macro_rules! plic_source_enable_addr {
    ($hart:expr, $mode:expr) => {{
        let hart_shift = ($hart) * 0x100;
        let mode_shift = if $mode == PrivilegeMode::Supervisor {
            0x80
        } else {
            0
        };
        unsafe { (PLIC_SOURCE_ENABLE as *mut u8).add((hart_shift + mode_shift)) as *mut u32 }
    }};
}

macro_rules! plic_threshold_addr {
    ($hart:expr, $mode:expr) => {{
        let hart_shift = ($hart) * 0x2000;
        let mode_shift = if $mode == PrivilegeMode::Supervisor {
            0x1000
        } else {
            0
        };
        unsafe { (PLIC_PRIORITY_THRESHOLD as *mut u8).add((hart_shift + mode_shift)) as *mut u32 }
    }};
}

macro_rules! plic_claim_complete_addr {
    ($hart:expr, $mode:expr) => {{
        let hart_shift = ($hart) * 0x2000;
        let mode_shift = if $mode == PrivilegeMode::Supervisor {
            0x1000
        } else {
            0
        };
        unsafe { (PLIC_CLAIM_COMPLETE as *mut u8).add((hart_shift + mode_shift)) as *mut u32 }
    }};
}

macro_rules! plic_set_priority {
    ($irq:expr, $priority:expr) => {{
        let addr = plic_source_priority_addr!($irq);
        unsafe {
            core::ptr::write_volatile(addr, ($priority as u32) & 0x7);
        }
    }};
}

macro_rules! plic_enable_irq {
    ($hart:expr, $mode:expr, $irq:expr) => {{
        let base_addr = plic_source_enable_addr!($hart, $mode);
        let irq = ($irq) as u64;
        let irq_shift = irq >> 5;
        let irq_bit = irq & 0x1f;
        unsafe {
            let addr = (base_addr as *mut u32).add(irq_shift as usize);
            let current = core::ptr::read_volatile(addr);
            core::ptr::write_volatile(addr, current | (1 << irq_bit));
        }
    }};
}

macro_rules! plic_set_hart_threshold {
    ($hart:expr, $mode:expr, $threshold:expr) => {{
        let addr = plic_threshold_addr!($hart, $mode);
        unsafe {
            core::ptr::write_volatile(addr, ($threshold as u32) & 0x7);
        }
    }};
}

pub fn plic_init() {
    plic_set_priority!(UART0_IRQ, 1);
    plic_enable_irq!(0, crate::riscv::PrivilegeMode::Supervisor, UART0_IRQ);
    plic_set_hart_threshold!(0, crate::riscv::PrivilegeMode::Supervisor, 0);
}

pub fn plic_claim() -> usize {
    let claim_addr = plic_claim_complete_addr!(0, crate::riscv::PrivilegeMode::Supervisor);
    let irq_id = unsafe { core::ptr::read_volatile(claim_addr) } as usize;
    irq_id
}

pub fn plic_complete(irq_id: usize) {
    let complete_addr = plic_claim_complete_addr!(0, crate::riscv::PrivilegeMode::Supervisor);
    unsafe {
        core::ptr::write_volatile(complete_addr, irq_id as u32);
    }
}
