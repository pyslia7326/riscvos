use core::cell::UnsafeCell;

use crate::csr;
use crate::riscv::PrivilegeMode;
use crate::syscall::sys_exit;
use crate::task::Stack;
use crate::task::TaskState;
use crate::task::TaskStruct;
use crate::task::USER_STACK_SIZE;
use crate::timer::get_current_tick;
use crate::uart::print_string;
use crate::utils::list::LinkedList;
use crate::utils::rc::Arc;

pub type RawTaskFn = fn(argc: u64, argv: &[&str]);
pub static SCHEDULER: SafeStaticScheduler = SafeStaticScheduler {
    inner: UnsafeCell::new(Scheduler::new()),
};

macro_rules! align_stack_ptr_16 {
    ($task_struct: expr) => {
        unsafe {
            const ALIGNMENT: u64 = 16;
            match ($task_struct).stack_ptr.as_ref() {
                Some(s) => {
                    let stack_ptr = s.get_ref().stack;
                    let stack_size = s.get_ref().size;
                    let stack_bottom = stack_ptr.as_ptr();
                    let stack_top = stack_bottom.add(stack_size).sub(ALIGNMENT as usize) as u64;
                    let remainder = stack_top & 0xf;
                    let padding = if remainder == 0 {
                        0
                    } else {
                        ALIGNMENT - remainder
                    };
                    Some((stack_top + padding) as u64)
                }
                None => None,
            }
        }
    };
}

pub struct Scheduler {
    pub running_list: Option<LinkedList<TaskStruct>>,
    pub waiting_list: Option<LinkedList<TaskStruct>>,
    pub blocked_list: Option<LinkedList<TaskStruct>>,
    pub pool: Option<LinkedList<TaskStruct>>,
    pub kernel_task: Option<TaskStruct>,
    pub idle_task: Option<TaskStruct>,
    pub new_task_id: u64,
}

pub struct SafeStaticScheduler {
    pub inner: UnsafeCell<Scheduler>,
}

unsafe impl Sync for SafeStaticScheduler {}

impl Scheduler {
    pub const fn new() -> Self {
        Self {
            running_list: None,
            waiting_list: None,
            blocked_list: None,
            pool: None,
            kernel_task: None,
            idle_task: None,
            new_task_id: 1,
        }
    }
}

fn idle_task() {
    loop {
        crate::wfi!();
    }
}

pub fn init() {
    let scheduler = unsafe { &mut *SCHEDULER.inner.get() };
    scheduler.running_list = Some(LinkedList::new());
    scheduler.waiting_list = Some(LinkedList::new());
    scheduler.blocked_list = Some(LinkedList::new());
    scheduler.pool = Some(LinkedList::new());
    // create kernel task struct
    csr::write_mscratch(
        scheduler.kernel_task.get_or_insert(TaskStruct::new()) as *const TaskStruct as u64,
    );
    // create idle task stack
    let mut idle_task_stack = match Stack::new(USER_STACK_SIZE) {
        Some(s) => Arc::new(s),
        None => {
            print_string("Scheduler init: create idle task stack failed\n");
            return;
        }
    };
    let idle_task_struct = scheduler.idle_task.get_or_insert(TaskStruct::new());
    idle_task_struct.stack_ptr = idle_task_stack.take();
    if let Some(sp) = align_stack_ptr_16!(idle_task_struct) {
        idle_task_struct.sp = sp;
    } else {
        print_string("Scheduler init: allign idle task sp failed\n");
        return;
    }
    idle_task_struct.xepc = idle_task as u64;
    csr::write_sepc(idle_task_struct.xepc);
    csr::write_sscratch(idle_task_struct as *const TaskStruct as u64);
    idle_task_struct.xepc = idle_task as u64;
    scheduler.new_task_id = 1;
    print_string("Scheduler init success\n");
}

pub fn task_start(task: RawTaskFn, args: *const u8, len: usize) {
    use core::slice;
    use core::str;
    if args.is_null() && len > 0 {
        return sys_exit(0);
    }
    let args = unsafe { slice::from_raw_parts(args, len) };
    let s = match str::from_utf8(args) {
        Ok(s) => s.trim_end_matches('\0'),
        Err(_) => return sys_exit(0),
    };
    let mut argv_buf: [&str; 5] = [""; 5];
    let mut argc = 0;
    for token in s.split(' ') {
        if token.is_empty() {
            continue;
        }
        if argc >= 5 {
            break;
        }
        argv_buf[argc] = token;
        argc += 1;
    }
    task(argc as u64, &argv_buf[..argc]);
    sys_exit(0);
}

pub fn task_create(task: *const u8, args: *const u8, len: usize) -> Option<u64> {
    let scheduler = unsafe { &mut *SCHEDULER.inner.get() };
    let pool = match scheduler.pool.as_mut() {
        Some(list) => list,
        None => return None,
    };
    let running = match scheduler.running_list.as_mut() {
        Some(list) => list,
        None => return None,
    };
    let new_task = if pool.is_empty() {
        let new_task_stack = match Stack::new(USER_STACK_SIZE) {
            Some(s) => Arc::new(s),
            None => return None,
        };
        let new_task_node = match pool.empty_node() {
            Some(n) => n,
            None => return None,
        };
        {
            let mut node_guard = new_task_node.get_ref().lock();
            let task_struct = node_guard.value.get_or_insert_with(|| TaskStruct::new());
            task_struct.stack_ptr = new_task_stack;
        }
        new_task_node
    } else {
        match pool.pop_front() {
            Some(n) => n,
            None => return None,
        }
    };
    let id = {
        let mut new_task_guard = new_task.get_ref().lock();
        let new_task_struct = match new_task_guard.value.as_mut() {
            Some(t) => t,
            None => return None,
        };
        new_task_struct.state = TaskState::Ready;
        new_task_struct.id = Some(scheduler.new_task_id);
        scheduler.new_task_id += 1;
        if let Some(sp) = align_stack_ptr_16!(new_task_struct) {
            new_task_struct.sp = sp;
        } else {
            return None;
        }
        new_task_struct.xepc = task_start as u64;
        new_task_struct.a[0] = task as u64;
        new_task_struct.a[1] = args as u64;
        new_task_struct.a[2] = len as u64;
        new_task_struct.id
    };
    running.push_back_node(new_task);
    id
}

pub fn schedule() {
    let scheduler = unsafe { &mut *SCHEDULER.inner.get() };
    let running_is_empty = {
        let running = match scheduler.running_list.as_mut() {
            Some(p) => p,
            None => panic!("Scheduler list should not be None\n"),
        };
        running.is_empty()
    };
    if running_is_empty {
        let mut tmp = scheduler.running_list.take();
        scheduler.running_list = scheduler.waiting_list.take();
        scheduler.waiting_list = tmp.take();
    }
    let blocked = match scheduler.blocked_list.as_mut() {
        Some(p) => p,
        None => panic!("Scheduler list should not be None\n"),
    };
    let running = match scheduler.running_list.as_mut() {
        Some(p) => p,
        None => panic!("Scheduler list should not be None\n"),
    };
    let waiting = match scheduler.waiting_list.as_mut() {
        Some(p) => p,
        None => panic!("Scheduler list should not be None\n"),
    };
    let pool = match scheduler.pool.as_mut() {
        Some(p) => p,
        None => panic!("Scheduler list should not be None\n"),
    };
    for b in blocked.iter_safe().unwrap() {
        let is_ready = {
            let mut guard = b.get_ref().lock();
            let btask = match guard.value.as_mut() {
                Some(t) => t,
                None => continue,
            };
            if let Some(sleep_until) = btask.sleep_until {
                if get_current_tick() >= sleep_until {
                    btask.state = TaskState::Ready;
                    btask.sleep_until = None;
                }
            }
            btask.state == TaskState::Ready
        };
        if is_ready {
            match LinkedList::remove_node_safe(b) {
                Some(n) => running.push_back_node(n),
                None => continue,
            };
        }
    }
    for r in running.iter_safe().unwrap() {
        let (xepc, struct_ptr, state) = {
            let mut guard = r.get_ref().lock();
            let rtask = match guard.value.as_mut() {
                Some(t) => t,
                None => continue,
            };
            match rtask.state {
                TaskState::Ready => rtask.state = TaskState::Ready,
                TaskState::Running => rtask.state = TaskState::Ready,
                TaskState::Sleeping => rtask.state = TaskState::Sleeping,
                _ => rtask.state = TaskState::None,
            }
            (rtask.xepc, rtask as *const TaskStruct as u64, rtask.state)
        };
        let task = match LinkedList::remove_node_safe(r) {
            Some(n) => n,
            None => continue,
        };
        match state {
            TaskState::Ready | TaskState::Running => {
                csr::write_sepc(xepc);
                csr::write_sscratch(struct_ptr);
                csr::sstatus_set_pp(PrivilegeMode::User);
                waiting.push_back_node(task);
                return;
            }
            TaskState::Sleeping => {
                blocked.push_back_node(task);
            }
            _ => {
                pool.push_back_node(task);
            }
        };
    }
    // if no task, switch to idle task.
    csr::write_sepc(scheduler.idle_task.as_ref().unwrap().xepc);
    csr::write_sscratch(scheduler.idle_task.as_ref().unwrap() as *const TaskStruct as u64);
    csr::sstatus_set_pp(PrivilegeMode::Supervisor);
}

pub fn get_task_state(id: u64) -> TaskState {
    let scheduler = unsafe { &mut *SCHEDULER.inner.get() };
    let blocked = scheduler.blocked_list.as_mut();
    let waiting = scheduler.waiting_list.as_mut();
    let running = scheduler.running_list.as_mut();
    if let Some(blist) = blocked {
        if let Some(iter) = blist.iter() {
            for task in iter {
                let guard = task.get_ref().lock();
                if let Some(t) = guard.value.as_ref() {
                    if let Some(tid) = t.id {
                        if tid == id {
                            // return t.state;
                            return TaskState::Sleeping;
                        }
                    }
                }
            }
        }
    }
    if let Some(wlist) = waiting {
        if let Some(iter) = wlist.iter() {
            for task in iter {
                let guard = task.get_ref().lock();
                if let Some(t) = guard.value.as_ref() {
                    if let Some(tid) = t.id {
                        if tid == id {
                            return t.state;
                        }
                    }
                }
            }
        }
    }
    if let Some(rlist) = running {
        if let Some(iter) = rlist.iter() {
            for task in iter {
                let guard = task.get_ref().lock();
                if let Some(t) = guard.value.as_ref() {
                    if let Some(tid) = t.id {
                        if tid == id {
                            return t.state;
                        }
                    }
                }
            }
        }
    }
    TaskState::None
}
