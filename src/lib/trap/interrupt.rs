pub const INTERRUPT_BIT: u64 = 1 << 63;

// Reserved: 0
pub const SUPERVISOR_SOFTWARE_INTERRUPT: u64 = 1 | INTERRUPT_BIT;
// Reserved: 2
pub const MACHINE_SOFTWARE_INTERRUPT: u64 = 3 | INTERRUPT_BIT;
// Reserved: 4
pub const SUPERVISOR_TIMER_INTERRUPT: u64 = 5 | INTERRUPT_BIT;
// Reserved: 6
pub const MACHINE_TIMER_INTERRUPT: u64 = 7 | INTERRUPT_BIT;
// Reserved: 8
pub const SUPERVISOR_EXTERNAL_INTERRUPT: u64 = 9 | INTERRUPT_BIT;
// Reserved: 10
pub const MACHINE_EXTERNAL_INTERRUPT: u64 = 11 | INTERRUPT_BIT;
// Reserved: 12
pub const COUNTER_OVERFLOW_INTERRUPT: u64 = 13 | INTERRUPT_BIT;
// Reserved: 14-15
// Designated for platform use: ≥16

macro_rules! __ENABLE_ALL_INTERRUPTS_BIT {
    () => {
        (1 << (SUPERVISOR_SOFTWARE_INTERRUPT & !INTERRUPT_BIT))
            | (1 << (MACHINE_SOFTWARE_INTERRUPT & !INTERRUPT_BIT))
            | (1 << (SUPERVISOR_TIMER_INTERRUPT & !INTERRUPT_BIT))
            | (1 << (MACHINE_TIMER_INTERRUPT & !INTERRUPT_BIT))
            | (1 << (SUPERVISOR_EXTERNAL_INTERRUPT & !INTERRUPT_BIT))
            | (1 << (MACHINE_EXTERNAL_INTERRUPT & !INTERRUPT_BIT))
            | (1 << (COUNTER_OVERFLOW_INTERRUPT & !INTERRUPT_BIT))
    };
}

pub const ENABLE_ALL_INTERRUPTS: u64 = __ENABLE_ALL_INTERRUPTS_BIT!();
