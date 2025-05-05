pub const INSTRUCTION_ADDRESS_MISALIGNED: u64 = 0;
pub const INSTRUCTION_ACCESS_FAULT: u64 = 1;
pub const ILLEGAL_INSTRUCTION: u64 = 2;
pub const BREAKPOINT: u64 = 3;
pub const LOAD_ADDRESS_MISALIGNED: u64 = 4;
pub const LOAD_ACCESS_FAULT: u64 = 5;
pub const STORE_AMO_ADDRESS_MISALIGNED: u64 = 6;
pub const STORE_AMO_ACCESS_FAULT: u64 = 7;
pub const ENVIRONMENT_CALL_FROM_U_MODE: u64 = 8;
pub const ENVIRONMENT_CALL_FROM_S_MODE: u64 = 9;
// Reserved: 10
pub const ENVIRONMENT_CALL_FROM_M_MODE: u64 = 11;
pub const INSTRUCTION_PAGE_FAULT: u64 = 12;
pub const LOAD_PAGE_FAULT: u64 = 13;
// Reserved: 14
pub const STORE_AMO_PAGE_FAULT: u64 = 15;
pub const DOUBLE_TRAP: u64 = 16;
// Reserved: 17
pub const SOFTWARE_CHECK: u64 = 18;
pub const HARDWARE_ERROR: u64 = 19;
// Reserved: 20-23
// Designated for custom use: 24-31
// Reserved: 32-47
// Designated for custom use: 48-63
// Reserved: ≥64

macro_rules! __ENABLE_ALL_EXCEPTIONS_BIT {
    () => {
        (1 << INSTRUCTION_ADDRESS_MISALIGNED)
            | (1 << INSTRUCTION_ACCESS_FAULT)
            | (1 << ILLEGAL_INSTRUCTION)
            | (1 << BREAKPOINT)
            | (1 << LOAD_ADDRESS_MISALIGNED)
            | (1 << LOAD_ACCESS_FAULT)
            | (1 << STORE_AMO_ADDRESS_MISALIGNED)
            | (1 << STORE_AMO_ACCESS_FAULT)
            | (1 << ENVIRONMENT_CALL_FROM_U_MODE)
            | (1 << ENVIRONMENT_CALL_FROM_S_MODE)
            | (1 << ENVIRONMENT_CALL_FROM_M_MODE)
            | (1 << INSTRUCTION_PAGE_FAULT)
            | (1 << LOAD_PAGE_FAULT)
            | (1 << STORE_AMO_PAGE_FAULT)
        // | (1 << DOUBLE_TRAP)
        // | (1 << SOFTWARE_CHECK)
        // | (1 << HARDWARE_ERROR)
    };
}

pub const ENABLE_ALL_EXCEPTIONS: u64 = __ENABLE_ALL_EXCEPTIONS_BIT!();
