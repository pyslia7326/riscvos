use core::arch::asm;
use paste::paste;

macro_rules! define_csr {
    ($csr_name:ident) => {
        paste! {
            #[inline(always)]
            pub fn [<read_ $csr_name>]() -> u64 {
                let value: u64;
                unsafe {
                    asm!(
                        concat!("csrr {0}, ", stringify!($csr_name)),
                        out(reg) value,
                        options(nomem, nostack, preserves_flags)
                    );
                }
                value
            }

            #[inline(always)]
            pub fn [<write_ $csr_name>](value: u64) {
                unsafe {
                    asm!(
                        concat!("csrw ", stringify!($csr_name), ", {0}"),
                        in(reg) value,
                        options(nomem, nostack, preserves_flags)
                    );
                }
            }
        }
    };
}

define_csr!(mstatus);
define_csr!(medeleg);
define_csr!(mideleg);
define_csr!(mie);
define_csr!(mtvec);
define_csr!(mscratch);
define_csr!(mepc);
define_csr!(mcause);
define_csr!(mtval);
define_csr!(mip);

define_csr!(sstatus);
define_csr!(sie);
define_csr!(stvec);
define_csr!(sscratch);
define_csr!(sepc);
define_csr!(scause);
define_csr!(stval);
define_csr!(sip);

define_csr!(pmpaddr0);
define_csr!(pmpcfg0);

pub const MSTATUS_MIE: u64 = 1 << 3;
pub const MSTATUS_MPIE: u64 = 1 << 7;
pub const MSTATUS_MPP: u64 = 1 << 11;
pub const MSTATUS_MPP_MASK: u64 = 0b11 << 11;

pub const MIE_MSIE: u64 = 1 << 3;
pub const MIE_MTIE: u64 = 1 << 7;
pub const MIE_MEIE: u64 = 1 << 11;

pub const MIP_MSIP: u64 = 1 << 3;
pub const MIP_MTIP: u64 = 1 << 7;
pub const MIP_MEIP: u64 = 1 << 11;

pub const SSTATUS_SIE: u64 = 1 << 1;
pub const SSTATUS_SPIE: u64 = 1 << 5;
pub const SSTATUS_SPP: u64 = 1 << 8;
pub const SSTATUS_SPP_MASK: u64 = 0b1 << 8;

pub const SIE_SSIE: u64 = 1 << 1;
pub const SIE_STIE: u64 = 1 << 5;
pub const SIE_SEIE: u64 = 1 << 9;

pub const SIP_SSIP: u64 = 1 << 1;
pub const SIP_STIP: u64 = 1 << 5;
pub const SIP_SEIP: u64 = 1 << 9;
