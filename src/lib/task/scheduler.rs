use crate::csr;
use crate::riscv::PrivilegeMode;
use crate::task::MAX_TASK_NUM;
use crate::task::Stack;
use crate::task::TaskState;
use crate::task::TaskStruct;
use crate::task::USER_STACK_SIZE;
use crate::uart::print_string;

static mut TASKS: [TaskStruct; MAX_TASK_NUM] = [TaskStruct::new(); MAX_TASK_NUM];
static mut TASK_STACKS: [Stack; MAX_TASK_NUM] = [Stack::new(); MAX_TASK_NUM];
static mut IDLE_TASK: [TaskStruct; 1] = [TaskStruct::new(); 1];
static mut IDLE_STACK: [Stack; 1] = [Stack::new(); 1];
static mut KERNEL_TASK: [TaskStruct; 1] = [TaskStruct::new(); 1];
static mut CURRENT_TASK: usize = 0;

pub fn get_kernel_task_struct() -> &'static mut TaskStruct {
    unsafe {
        let task_struct = &mut KERNEL_TASK[0];
        task_struct
    }
}

pub fn get_current_task() -> usize {
    unsafe { CURRENT_TASK }
}

pub fn set_current_task(id: usize) {
    unsafe {
        CURRENT_TASK = id;
    }
}

pub fn get_stack_ptr(id: usize) -> *mut u8 {
    unsafe {
        let stack = TASK_STACKS[id].stack.as_mut_ptr();
        let stack_top = stack.add(USER_STACK_SIZE).sub(8);
        stack_top
    }
}

pub fn get_task_struct(id: usize) -> &'static mut TaskStruct {
    unsafe {
        let task_struct = &mut TASKS[id];
        task_struct
    }
}

pub fn create_task(task: fn()) -> Option<&'static TaskStruct> {
    for i in 0..MAX_TASK_NUM {
        let task_struct = get_task_struct(i);
        if task_struct.state == TaskState::None {
            task_struct.state = TaskState::Ready;
            task_struct.id = Some(i as u64);
            task_struct.stack_ptr = get_stack_ptr(i);
            task_struct.sp = get_stack_ptr(i) as u64;
            task_struct.xepc = task as u64;
            return Some(task_struct);
        }
    }
    None
}

fn idle_task() {
    print_string("Idle task is running!\n");
    // crate::ecall!();
    loop {
        crate::wfi!();
    }
}

pub fn get_idle_stack_ptr() -> *mut u8 {
    unsafe {
        let stack = IDLE_STACK[0].stack.as_mut_ptr();
        let stack_top = stack.add(USER_STACK_SIZE).sub(8);
        stack_top
    }
}

pub fn get_idle_task_struct() -> &'static mut TaskStruct {
    unsafe {
        let task_struct = &mut IDLE_TASK[0];
        task_struct
    }
}

pub fn create_idle_task() -> Option<&'static TaskStruct> {
    let task_struct = get_idle_task_struct();
    task_struct.state = TaskState::Ready;
    task_struct.stack_ptr = get_idle_stack_ptr();
    task_struct.sp = get_idle_stack_ptr() as u64;
    task_struct.xepc = idle_task as u64;
    Some(task_struct)
}

pub fn scheduler() -> &'static TaskStruct {
    let current_id = get_current_task();
    for i in 0..MAX_TASK_NUM {
        let next_id = (current_id + i + 1) % MAX_TASK_NUM;
        let next_task = get_task_struct(next_id);
        if next_task.state == TaskState::Ready {
            set_current_task(next_id);
            next_task.state = TaskState::Running;
            csr::write_sscratch(next_task as *const TaskStruct as u64);
            csr::sstatus_set_pp(PrivilegeMode::User);
            return next_task;
        }
    }
    let idle_task = get_idle_task_struct();
    csr::sstatus_set_pp(PrivilegeMode::Supervisor);
    csr::write_sscratch(idle_task as *const TaskStruct as u64);
    idle_task
}
