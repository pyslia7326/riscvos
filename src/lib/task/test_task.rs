use crate::syscall::sys_sleep;

pub fn user_task1(_argc: u64, _argv: *const *const u8) {
    loop {
        sys_sleep(1000);
    }
}
