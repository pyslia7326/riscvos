use crate::csr;
use crate::task::scheduler::scheduler;
use crate::task::{TaskState, TaskStruct};
use crate::trap::exception;
use crate::trap::interrupt;

#[no_mangle]
#[naked_function::naked]
pub unsafe extern "C" fn trap_handler() {
    asm! (
        // Save all general-purpose registers and the return address to the current task's stack frame.
        "csrrw a0, sscratch, a0", // a0 holds the task_struct pointer, sscratch holds the original a0 value
        "sd ra, {offset_ra}(a0)",
        "sd sp, {offset_sp}(a0)",
        "sd gp, {offset_gp}(a0)",
        "sd tp, {offset_tp}(a0)",
        "sd t0, {offset_t0}(a0)",
        "sd t1, {offset_t1}(a0)",
        "sd t2, {offset_t2}(a0)",
        "sd t3, {offset_t3}(a0)",
        "sd t4, {offset_t4}(a0)",
        "sd t5, {offset_t5}(a0)",
        "sd t6, {offset_t6}(a0)",
        "sd s0, {offset_s0}(a0)",
        "sd s1, {offset_s1}(a0)",
        "sd s2, {offset_s2}(a0)",
        "sd s3, {offset_s3}(a0)",
        "sd s4, {offset_s4}(a0)",
        "sd s5, {offset_s5}(a0)",
        "sd s6, {offset_s6}(a0)",
        "sd s7, {offset_s7}(a0)",
        "sd s8, {offset_s8}(a0)",
        "sd s9, {offset_s9}(a0)",
        "sd s10, {offset_s10}(a0)",
        "sd s11, {offset_s11}(a0)",
        "sd a1, {offset_a1}(a0)",
        "sd a2, {offset_a2}(a0)",
        "sd a3, {offset_a3}(a0)",
        "sd a4, {offset_a4}(a0)",
        "sd a5, {offset_a5}(a0)",
        "sd a6, {offset_a6}(a0)",
        "sd a7, {offset_a7}(a0)",
        "csrr a1, sepc",
        "sd a1, {offset_sepc}(a0)",
        "csrrw a1, sscratch, a0", // a1 holds the original a0 value, a0 and sscratch hold the task_struct pointer
        "sd a1, {offset_a0}(a0)",
        "call trap_dispatch",
        "j trap_return",
        offset_ra = const crate::task::OFFSET_RA,
        offset_sp = const crate::task::OFFSET_SP,
        offset_gp = const crate::task::OFFSET_GP,
        offset_tp = const crate::task::OFFSET_TP,
        offset_t0 = const crate::task::OFFSET_T0,
        offset_t1 = const crate::task::OFFSET_T1,
        offset_t2 = const crate::task::OFFSET_T2,
        offset_t3 = const crate::task::OFFSET_T3,
        offset_t4 = const crate::task::OFFSET_T4,
        offset_t5 = const crate::task::OFFSET_T5,
        offset_t6 = const crate::task::OFFSET_T6,
        offset_s0 = const crate::task::OFFSET_S0,
        offset_s1 = const crate::task::OFFSET_S1,
        offset_s2 = const crate::task::OFFSET_S2,
        offset_s3 = const crate::task::OFFSET_S3,
        offset_s4 = const crate::task::OFFSET_S4,
        offset_s5 = const crate::task::OFFSET_S5,
        offset_s6 = const crate::task::OFFSET_S6,
        offset_s7 = const crate::task::OFFSET_S7,
        offset_s8 = const crate::task::OFFSET_S8,
        offset_s9 = const crate::task::OFFSET_S9,
        offset_s10 = const crate::task::OFFSET_S10,
        offset_s11 = const crate::task::OFFSET_S11,
        offset_a0 = const crate::task::OFFSET_A0,
        offset_a1 = const crate::task::OFFSET_A1,
        offset_a2 = const crate::task::OFFSET_A2,
        offset_a3 = const crate::task::OFFSET_A3,
        offset_a4 = const crate::task::OFFSET_A4,
        offset_a5 = const crate::task::OFFSET_A5,
        offset_a6 = const crate::task::OFFSET_A6,
        offset_a7 = const crate::task::OFFSET_A7,
        offset_sepc = const crate::task::OFFSET_SEPC,
    );
}

#[no_mangle]
#[naked_function::naked]
pub unsafe extern "C" fn trap_return() {
    asm! (
        // Restore all general-purpose registers and return to the interrupt or exception.
        "csrr a7, sscratch",
        "ld ra, {offset_ra}(a7)",
        "ld sp, {offset_sp}(a7)",
        "ld gp, {offset_gp}(a7)",
        "ld tp, {offset_tp}(a7)",
        "ld t0, {offset_t0}(a7)",
        "ld t1, {offset_t1}(a7)",
        "ld t2, {offset_t2}(a7)",
        "ld t3, {offset_t3}(a7)",
        "ld t4, {offset_t4}(a7)",
        "ld t5, {offset_t5}(a7)",
        "ld t6, {offset_t6}(a7)",
        "ld s0, {offset_s0}(a7)",
        "ld s1, {offset_s1}(a7)",
        "ld s2, {offset_s2}(a7)",
        "ld s3, {offset_s3}(a7)",
        "ld s4, {offset_s4}(a7)",
        "ld s5, {offset_s5}(a7)",
        "ld s6, {offset_s6}(a7)",
        "ld s7, {offset_s7}(a7)",
        "ld s8, {offset_s8}(a7)",
        "ld s9, {offset_s9}(a7)",
        "ld s10, {offset_s10}(a7)",
        "ld s11, {offset_s11}(a7)",
        "ld a0, {offset_a0}(a7)",
        "ld a1, {offset_a1}(a7)",
        "ld a2, {offset_a2}(a7)",
        "ld a3, {offset_a3}(a7)",
        "ld a4, {offset_a4}(a7)",
        "ld a5, {offset_a5}(a7)",
        "ld a6, {offset_a6}(a7)",
        "ld a7, {offset_a7}(a7)",
        "sret",
        offset_ra = const crate::task::OFFSET_RA,
        offset_sp = const crate::task::OFFSET_SP,
        offset_gp = const crate::task::OFFSET_GP,
        offset_tp = const crate::task::OFFSET_TP,
        offset_t0 = const crate::task::OFFSET_T0,
        offset_t1 = const crate::task::OFFSET_T1,
        offset_t2 = const crate::task::OFFSET_T2,
        offset_t3 = const crate::task::OFFSET_T3,
        offset_t4 = const crate::task::OFFSET_T4,
        offset_t5 = const crate::task::OFFSET_T5,
        offset_t6 = const crate::task::OFFSET_T6,
        offset_s0 = const crate::task::OFFSET_S0,
        offset_s1 = const crate::task::OFFSET_S1,
        offset_s2 = const crate::task::OFFSET_S2,
        offset_s3 = const crate::task::OFFSET_S3,
        offset_s4 = const crate::task::OFFSET_S4,
        offset_s5 = const crate::task::OFFSET_S5,
        offset_s6 = const crate::task::OFFSET_S6,
        offset_s7 = const crate::task::OFFSET_S7,
        offset_s8 = const crate::task::OFFSET_S8,
        offset_s9 = const crate::task::OFFSET_S9,
        offset_s10 = const crate::task::OFFSET_S10,
        offset_s11 = const crate::task::OFFSET_S11,
        offset_a0 = const crate::task::OFFSET_A0,
        offset_a1 = const crate::task::OFFSET_A1,
        offset_a2 = const crate::task::OFFSET_A2,
        offset_a3 = const crate::task::OFFSET_A3,
        offset_a4 = const crate::task::OFFSET_A4,
        offset_a5 = const crate::task::OFFSET_A5,
        offset_a6 = const crate::task::OFFSET_A6,
        offset_a7 = const crate::task::OFFSET_A7,
    );
}

#[unsafe(no_mangle)]
pub fn trap_dispatch(cur_task_struct: &mut TaskStruct) {
    let scause = csr::read_scause();
    cur_task_struct.state = TaskState::Ready;
    match scause {
        interrupt::SUPERVISOR_TIMER_INTERRUPT => {
            // scheduler
        }
        exception::ENVIRONMENT_CALL_FROM_U_MODE => {
            // Increment sepc to the instruction after ecall.
            cur_task_struct.sepc += 4;
            let next_task_struct = scheduler();
            csr::write_sepc(next_task_struct.sepc);
            csr::write_sscratch(next_task_struct as *const TaskStruct as u64);
            return;
        }
        _ => {}
    }
    // Resume the current task
    cur_task_struct.state = TaskState::Running;
    csr::write_sepc(cur_task_struct.sepc);
    csr::write_sscratch(cur_task_struct as *const TaskStruct as u64);
    return;
}
