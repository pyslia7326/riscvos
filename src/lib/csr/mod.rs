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

pub const MSTATUS_MIE: u8 = 3;
pub const MSTATUS_MPIE: u8 = 7;
pub const MSTATUS_MPP: u8 = 11;
pub const MSTATUS_MPP_MASK: u64 = 0b11 << MSTATUS_MPP;

pub const MIE_MSIE: u8 = 3;
pub const MIE_MTIE: u8 = 7;
pub const MIE_MEIE: u8 = 11;

pub const MIP_MSIP: u8 = 3;
pub const MIP_MTIP: u8 = 7;
pub const MIP_MEIP: u8 = 11;

pub const SSTATUS_SIE: u8 = 1;
pub const SSTATUS_SPIE: u8 = 5;
pub const SSTATUS_SPP: u8 = 8;
pub const SSTATUS_SPP_MASK: u64 = 0b1 << SSTATUS_SPP;

pub const SIE_SSIE: u8 = 1;
pub const SIE_STIE: u8 = 5;
pub const SIE_SEIE: u8 = 9;

pub const SIP_SSIP: u8 = 1;
pub const SIP_STIP: u8 = 5;
pub const SIP_SEIP: u8 = 9;
