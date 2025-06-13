use crate::task::scheduler;
use crate::task::{TaskState, TaskStruct};
use crate::timer::get_current_tick;
use crate::uart::{uart_read, uart_write};
use crate::utils::cstr::u64_to_str;
use crate::utils::malloc;

#[derive(Clone, Copy)]
#[repr(u64)]
pub enum Syscall {
    Yield = 0,
    Exit = 1,
    Sleep = 2,
    Write = 3,
    Read = 4,
    Wait = 5,
    Spawn = 6,
    Alloc = 7,
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
            4 => Syscall::Read,
            5 => Syscall::Wait,
            6 => Syscall::Spawn,
            7 => Syscall::Alloc,
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
        Syscall::Read => {
            task.state = TaskState::Ready;
            task.xepc += 4;
            let buf = task.a[0] as *mut u8;
            let len = task.a[1] as usize;
            let read_len = uart_read(buf, len).unwrap_or(0);
            task.a[0] = read_len as u64;
        }
        Syscall::Wait => {
            task.state = TaskState::Ready;
            let wait_id = task.a[0];
            if scheduler::get_task_state(wait_id) == TaskState::None {
                task.xepc += 4;
            }
        }
        Syscall::Spawn => {
            task.state = TaskState::Ready;
            task.xepc += 4;
            let task_ptr = task.a[0] as *const u8;
            let args = task.a[1] as *const u8;
            let len = task.a[2] as usize;
            task.a[0] = scheduler::task_create(task_ptr, args, len).unwrap_or(0);
        }
        Syscall::Alloc => {
            task.state = TaskState::Ready;
            task.xepc += 4;
            let nbytes = task.a[0] as usize;
            task.a[0] = unsafe { malloc::malloc(nbytes).unwrap_or(core::ptr::null_mut()) as u64 };
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
    let ptr = core::hint::black_box(s.as_ptr());
    let len = core::hint::black_box(s.len());
    unsafe {
        core::arch::asm!(
            "mv a7, {syscall_code}",
            "mv a0, {ptr}",
            "mv a1, {len}",
            "ecall",
            syscall_code = in(reg) Syscall::Write.code(),
            ptr = in(reg) ptr,
            len = in(reg) len,
        );
    }
}

pub fn sys_write_u64(num: u64) {
    let mut buffer = [0; 20];
    match u64_to_str(num, &mut buffer) {
        Ok(num) => unsafe {
            let ptr = core::hint::black_box(num.as_ptr());
            let len = core::hint::black_box(num.len());
            core::arch::asm!(
                "mv a7, {syscall_code}",
                "mv a0, {ptr}",
                "mv a1, {len}",
                "ecall",
                syscall_code = in(reg) Syscall::Write.code(),
                ptr = in(reg) ptr,
                len = in(reg) len,
            );
        },
        Err(_) => {
            //
        }
    }
}

pub fn sys_read(buf: &[u8]) -> Option<u64> {
    let mut read_len: u64;
    let ptr = core::hint::black_box(buf.as_ptr());
    let len = core::hint::black_box(buf.len());
    unsafe {
        core::arch::asm!(
            "mv a7, {syscall_code}",
            "mv a0, {ptr}",
            "mv a1, {len}",
            "ecall",
            "mv {read_len}, a0",
            syscall_code = in(reg) Syscall::Read.code(),
            ptr = in(reg) ptr,
            len = in(reg) len,
            read_len = out(reg) read_len,
        );
    }
    if read_len == 0 { None } else { Some(read_len) }
}

pub fn sys_wait(pid: usize) {
    unsafe {
        core::arch::asm!(
            "mv a7, {syscall_code}",
            "mv a0, {pid}",
            "ecall",
            syscall_code = in(reg) Syscall::Wait.code(),
            pid = in(reg) pid,
        );
    }
}

pub fn sys_spawn(task: fn(argc: u64, argv: &[&str]), args: *const u8, len: usize) -> Option<u64> {
    let mut id;
    unsafe {
        core::arch::asm!(
            "mv a7, {syscall_code}",
            "mv a0, {task}",
            "mv a1, {args}",
            "mv a2, {len}",
            "ecall",
            "mv {id}, a0",
            syscall_code = in(reg) Syscall::Spawn.code(),
            task = in(reg) task,
            args = in(reg) args,
            len = in(reg) len,
            id = out(reg) id,
        );
    }
    if id == 0 { None } else { Some(id) }
}

pub fn sys_alloc(nbytes: usize) -> Option<*mut u8> {
    let mut ptr: u64;
    unsafe {
        core::arch::asm!(
            "mv a7, {syscall_code}",
            "mv a0, {nbytes}",
            "ecall",
            "mv {ptr}, a0",
            syscall_code = in(reg) Syscall::Alloc.code(),
            nbytes = in(reg) nbytes,
            ptr = out(reg) ptr,
        );
    }
    if ptr == (core::ptr::null_mut::<u8>() as u64) {
        None
    } else {
        Some(ptr as *mut u8)
    }
}
