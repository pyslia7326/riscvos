use crate::csr;

const CLINT_BASE: u64 = 0x200_0000;
const _MSIP_BASE: u64 = CLINT_BASE;
const MTIMECMP_BASE: u64 = CLINT_BASE + 0x4000;
const MTIME_BASE: u64 = CLINT_BASE + 0xBFF8;

const INTERVAL: u64 = 10000000;

macro_rules! read_mtime {
    () => {
        unsafe { core::ptr::read_volatile(MTIME_BASE as *const u64) }
    };
}

macro_rules! write_mtimecmp {
    ($hart_id: expr, $value: expr) => {
        let mtimecmp_address = MTIMECMP_BASE + $hart_id * 8;
        unsafe {
            core::ptr::write_volatile(mtimecmp_address as *mut u64, $value);
        }
    };
}

pub fn timer_init() {
    let hart_id = csr::read_mhartid();
    let cur_time = read_mtime!();
    write_mtimecmp!(hart_id, cur_time + INTERVAL);
    csr::write_mstatus(csr::read_mstatus() | 1 << csr::MSTATUS_MIE);
    csr::write_mie(csr::read_mie() | 1 << csr::MIE_MTIE);
}

pub fn timer_handler() {
    csr::write_mip(csr::read_mip() | (1 << csr::MIP_MSIP));
    csr::write_sip(csr::read_sip() | (1 << csr::SIP_SSIP));

    let hart_id = csr::read_mhartid();
    let cur_time = read_mtime!();
    write_mtimecmp!(hart_id, cur_time + INTERVAL);
}
