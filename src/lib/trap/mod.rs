pub mod exception;
pub mod interrupt;
pub mod kernel_trap;
pub mod user_trap;

use crate::csr;
use crate::plic::{UART0_IRQ, plic_claim, plic_complete};
use crate::syscall::syscall_handler;
use crate::task::{TaskState, TaskStruct};
use crate::timer::timer_handler;
use crate::uart::{print_integer, print_string, uart_irq_handler, uart_write_buffer_flush};

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
            crate::task::scheduler::schedule();

            // let next_task_struct = scheduler();
            // csr::write_sepc(next_task_struct.xepc);
            // csr::write_sscratch(next_task_struct as *const TaskStruct as u64);
            csr::write_sip(csr::read_sip() & !(1 << csr::SIP_SSIP));
        }
        interrupt::SUPERVISOR_EXTERNAL_INTERRUPT => {
            let irq_id = plic_claim();
            match irq_id {
                UART0_IRQ => uart_irq_handler(),
                _ => print_string("not implement irq\n"),
            }
            if irq_id > 0 {
                plic_complete(irq_id);
            }
        }
        exception::ENVIRONMENT_CALL_FROM_U_MODE => {
            cur_task_struct.state = TaskState::Ready;
            syscall_handler(cur_task_struct);
            crate::task::scheduler::schedule();
            // let next_task_struct = scheduler();
            // csr::write_sepc(next_task_struct.xepc);
            // csr::write_sscratch(next_task_struct as *const TaskStruct as u64);
        }
        _ => {
            // Resume the current task
            print_string("<< ");
            print_integer(cur_task_struct.xcause);
            print_string(", ");
            print_integer(cur_task_struct.xepc);
            print_string(", ");
            print_integer(cur_task_struct.ra);
            print_string(" >>");
            panic!("");
            // cur_task_struct.state = TaskState::Running;
            // csr::write_sepc(cur_task_struct.xepc);
            // csr::write_sscratch(cur_task_struct as *const TaskStruct as u64);
        }
    }
    // Only flush UART buffer for non-m-mode timer interrupts.
    // If sys_write is holding the UART spin lock and a machine timer interrupt (m-mode) occurs,
    // the timer interrupt handler would also try to acquire the same lock to flush the buffer,
    // causing the interrupt handler to spin (busy wait) until the lock is released.
    // To avoid this potential deadlock/spin, we skip flushing in m-mode timer interrupts.
    if cause != interrupt::MACHINE_TIMER_INTERRUPT {
        uart_write_buffer_flush();
    }
}
