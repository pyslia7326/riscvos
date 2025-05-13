pub mod exception;
pub mod interrupt;
pub mod kernel_trap;
pub mod user_trap;

use crate::csr;
use crate::syscall::syscall_handler;
use crate::task::scheduler::scheduler;
use crate::task::{TaskState, TaskStruct};
use crate::timer::timer_handler;

#[unsafe(no_mangle)]
pub fn trap_dispatch(cur_task_struct: &mut TaskStruct) {
    let cause = cur_task_struct.xcause;
    match cause {
        interrupt::MACHINE_TIMER_INTERRUPT => {
            // print_string("==========================================\n");
            // print_string("  Triggered a machine timer interrupt!\n");
            // print_string("==========================================\n");
            // timer_handler will reassign a supervisor software interrupt
            timer_handler();
        }
        interrupt::SUPERVISOR_SOFTWARE_INTERRUPT => {
            // print_string("==========================================\n");
            // print_string("Triggered a supervisor software interrupt!\n");
            // print_string("==========================================\n");
            cur_task_struct.state = TaskState::Ready;
            let next_task_struct = scheduler();
            csr::write_sepc(next_task_struct.xepc);
            csr::write_sscratch(next_task_struct as *const TaskStruct as u64);
            csr::write_sip(csr::read_sip() & !(1 << csr::SIP_SSIP));
            // print_string("switch to task pid: ");
            // if let Some(pid) = next_task_struct.id {
            //     print_integerln(pid);
            // } else {
            //     print_string("idle\n");
            // }
        }
        exception::ENVIRONMENT_CALL_FROM_U_MODE => {
            // print_string("==========================================\n");
            // print_string("    Triggered a user environment call!\n");
            // print_string("==========================================\n");
            syscall_handler(cur_task_struct);
            let next_task_struct = scheduler();
            csr::write_sepc(next_task_struct.xepc);
            csr::write_sscratch(next_task_struct as *const TaskStruct as u64);
            // print_string("switch to task pid: ");
            // if let Some(pid) = next_task_struct.id {
            //     print_integerln(pid);
            // } else {
            //     print_string("idle\n");
            // }
        }
        // exception::ENVIRONMENT_CALL_FROM_S_MODE => {
        //     print_string("==========================================\n");
        //     print_string(" Triggered a supervisor environment call!\n");
        //     print_string("==========================================\n");
        //     // Increment sepc to the instruction after ecall.
        //     cur_task_struct.xepc += 4;
        //     let next_task_struct = scheduler();
        //     csr::write_sepc(next_task_struct.xepc);
        //     csr::write_sscratch(next_task_struct as *const TaskStruct as u64);
        //     print_string("switch to task pid: ");
        //     if let Some(pid) = next_task_struct.id {
        //         print_integerln(pid);
        //     } else {
        //         print_string("idle\n");
        //     }
        // }
        _ => {
            // Resume the current task
            cur_task_struct.state = TaskState::Running;
            csr::write_sepc(cur_task_struct.xepc);
            csr::write_sscratch(cur_task_struct as *const TaskStruct as u64);
        }
    }
}
