use crate::task::{TaskState, TaskStruct};
use crate::timer::get_current_tick;
use crate::uart::uart_write;

#[derive(Clone, Copy)]
#[repr(u64)]
pub enum Syscall {
    Yield = 0,
    Exit = 1,
    Sleep = 2,
    Write = 3,
    Unknown,
}

impl Syscall {
    pub fn code(&self) -> u64 {
        self.clone() as u64
    }

    pub fn from(code: u64) -> Syscall {
        match code {
            0 => Syscall::Yield,
            1 => Syscall::Exit,
            2 => Syscall::Sleep,
            3 => Syscall::Write,
            _ => Syscall::Unknown,
        }
    }
}

pub fn syscall_handler(task: &mut TaskStruct) {
    let syscall = Syscall::from(task.a[7]);
    match syscall {
        Syscall::Yield => {
            task.state = TaskState::Ready;
            task.xepc += 4;
        }
        Syscall::Exit => {
            // TODO: exit code
            task.state = TaskState::None;
        }
        Syscall::Sleep => {
            task.state = TaskState::Sleeping;
            task.xepc += 4;
            let cur_tick = get_current_tick();
            let sleep_ticks = task.a[0];
            task.sleep_until = Some(cur_tick + sleep_ticks);
        }
        Syscall::Write => {
            task.state = TaskState::Ready;
            task.xepc += 4;
            let ptr = task.a[0] as *const u8;
            let len = task.a[1] as usize;
            uart_write(ptr, len);
        }
        Syscall::Unknown => panic!("Unknown syscall code: {}", task.a[7]),
    }
}

pub fn sys_yield() {
    unsafe {
        core::arch::asm!(
            "mv a7, {}",
            "ecall",
            in(reg) Syscall::Yield.code(),
        );
    }
}

pub fn sys_exit(id: u64) {
    unsafe {
        core::arch::asm!(
            "mv a7, {}",
            "mv a0, {}",
            "ecall",
            in(reg) Syscall::Exit.code(),
            in(reg) id,
        );
    }
}

pub fn sys_sleep(ticks: u64) {
    unsafe {
        core::arch::asm!(
            "mv a7, {}",
            "mv a0, {}",
            "ecall",
            in(reg) Syscall::Sleep.code(),
            in(reg) ticks,
        );
    }
}

pub fn sys_write(s: &str) {
    unsafe {
        core::arch::asm!(
            "mv a7, {syscall_code}",
            "mv a0, {ptr}",
            "mv a1, {len}",
            "ecall",
            syscall_code = in(reg) Syscall::Write.code(),
            ptr = in(reg) s.as_ptr(),
            len = in(reg) s.len(),
        );
    }
}
