use core::{mem::offset_of, ptr::NonNull};

use crate::utils::malloc::{free, malloc};
use crate::utils::rc::Arc;

pub mod scheduler;
pub mod test_task;

const USER_STACK_ALIGNMENT: usize = 16;
const USER_STACK_SIZE: usize = 4096;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum TaskState {
    None,
    Ready,
    Running,
    Sleeping,
    Blocked,
}

#[derive(Clone)]
pub struct Stack {
    pub stack: NonNull<u8>,
    pub size: usize,
}

pub struct TaskStruct {
    pub id: Option<u64>,
    pub state: TaskState,
    pub stack_ptr: Option<Arc<Stack>>,
    pub sleep_until: Option<u64>,
    pub xepc: u64,
    pub xcause: u64,
    pub ra: u64,
    pub sp: u64,
    pub gp: u64,
    pub tp: u64,
    pub t: [u64; 7],
    pub s: [u64; 12],
    pub a: [u64; 8],
}

pub const OFFSET_XEPC: usize = offset_of!(TaskStruct, xepc);
pub const OFFSET_XCAUSE: usize = offset_of!(TaskStruct, xcause);

pub const OFFSET_RA: usize = offset_of!(TaskStruct, ra);
pub const OFFSET_SP: usize = offset_of!(TaskStruct, sp);
pub const OFFSET_GP: usize = offset_of!(TaskStruct, gp);
pub const OFFSET_TP: usize = offset_of!(TaskStruct, tp);
pub const OFFSET_T: usize = offset_of!(TaskStruct, t);
pub const OFFSET_S: usize = offset_of!(TaskStruct, s);
pub const OFFSET_A: usize = offset_of!(TaskStruct, a);

pub const OFFSET_T0: usize = OFFSET_T + 0 * 8;
pub const OFFSET_T1: usize = OFFSET_T + 1 * 8;
pub const OFFSET_T2: usize = OFFSET_T + 2 * 8;
pub const OFFSET_T3: usize = OFFSET_T + 3 * 8;
pub const OFFSET_T4: usize = OFFSET_T + 4 * 8;
pub const OFFSET_T5: usize = OFFSET_T + 5 * 8;
pub const OFFSET_T6: usize = OFFSET_T + 6 * 8;

pub const OFFSET_S0: usize = OFFSET_S + 0 * 8;
pub const OFFSET_S1: usize = OFFSET_S + 1 * 8;
pub const OFFSET_S2: usize = OFFSET_S + 2 * 8;
pub const OFFSET_S3: usize = OFFSET_S + 3 * 8;
pub const OFFSET_S4: usize = OFFSET_S + 4 * 8;
pub const OFFSET_S5: usize = OFFSET_S + 5 * 8;
pub const OFFSET_S6: usize = OFFSET_S + 6 * 8;
pub const OFFSET_S7: usize = OFFSET_S + 7 * 8;
pub const OFFSET_S8: usize = OFFSET_S + 8 * 8;
pub const OFFSET_S9: usize = OFFSET_S + 9 * 8;
pub const OFFSET_S10: usize = OFFSET_S + 10 * 8;
pub const OFFSET_S11: usize = OFFSET_S + 11 * 8;

pub const OFFSET_A0: usize = OFFSET_A + 0 * 8;
pub const OFFSET_A1: usize = OFFSET_A + 1 * 8;
pub const OFFSET_A2: usize = OFFSET_A + 2 * 8;
pub const OFFSET_A3: usize = OFFSET_A + 3 * 8;
pub const OFFSET_A4: usize = OFFSET_A + 4 * 8;
pub const OFFSET_A5: usize = OFFSET_A + 5 * 8;
pub const OFFSET_A6: usize = OFFSET_A + 6 * 8;
pub const OFFSET_A7: usize = OFFSET_A + 7 * 8;

impl Stack {
    pub fn new(nbytes: usize) -> Option<Self> {
        unsafe {
            Some(Self {
                stack: NonNull::new(malloc(nbytes)?)?,
                size: nbytes,
            })
        }
    }
}

impl Drop for Stack {
    fn drop(&mut self) {
        unsafe {
            free(self.stack.as_ptr());
        }
    }
}

impl TaskStruct {
    pub const fn new() -> Self {
        Self {
            id: None,
            state: TaskState::None,
            stack_ptr: None,
            sleep_until: None,
            xepc: 0,
            xcause: 0,
            ra: 0,
            sp: 0,
            gp: 0,
            tp: 0,
            t: [0; 7],
            s: [0; 12],
            a: [0; 8],
        }
    }
}
