use crate::csr;
use core::sync::atomic::{AtomicU64, Ordering};

const CLINT_BASE: u64 = 0x200_0000;
const _MSIP_BASE: u64 = CLINT_BASE;
const MTIMECMP_BASE: u64 = CLINT_BASE + 0x4000;
const MTIME_BASE: u64 = CLINT_BASE + 0xBFF8;

static INTERRUPT_INTERVAL: u64 = 10000;

const TICK_INTERVAL: u64 = 10000;
static CURRENT_TICK: AtomicU64 = AtomicU64::new(0);
static INITIAL_MTIME: AtomicU64 = AtomicU64::new(0);

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

pub fn get_current_tick() -> u64 {
    CURRENT_TICK.load(Ordering::Acquire)
}

pub fn set_current_tick(tick: u64) {
    CURRENT_TICK.store(tick, Ordering::Release);
}

pub fn get_tick_mtime() -> u64 {
    INITIAL_MTIME.load(Ordering::Acquire) + CURRENT_TICK.load(Ordering::Acquire) * TICK_INTERVAL
}

pub fn timer_init() {
    let hart_id = csr::read_mhartid();
    let cur_time = read_mtime!();
    INITIAL_MTIME.store(cur_time, Ordering::Release);
    write_mtimecmp!(hart_id, cur_time + INTERRUPT_INTERVAL);
    csr::write_mstatus(csr::read_mstatus() | 1 << csr::MSTATUS_MIE);
    csr::write_mie(csr::read_mie() | 1 << csr::MIE_MTIE);
}

pub fn timer_handler() {
    csr::write_mip(csr::read_mip() | (1 << csr::MIP_MSIP));
    csr::write_sip(csr::read_sip() | (1 << csr::SIP_SSIP));

    let hart_id = csr::read_mhartid();
    let cur_time = read_mtime!();
    write_mtimecmp!(hart_id, cur_time + INTERRUPT_INTERVAL);
    let initial_mtime = INITIAL_MTIME.load(Ordering::Acquire);
    let delta = cur_time - initial_mtime;
    set_current_tick(delta / TICK_INTERVAL);
}
