use crate::mutex::Mutex;
use crate::uart::print_integerln;

static mut TMP: u64 = 0;
static LOCK: Mutex = Mutex::new();

pub fn user_task1(_argc: u64, _argv: *const *const u8) {
    for _ in 0..1000000 {
        LOCK.lock();
        unsafe {
            TMP += 1;
        }
        LOCK.unlock();
    }
    LOCK.lock();
    print_integerln(unsafe { TMP });
    LOCK.unlock();
}

pub fn user_task2(_argc: u64, _argv: *const *const u8) {
    for _ in 0..1000000 {
        LOCK.lock();
        unsafe {
            TMP += 1;
        }
        LOCK.unlock();
    }
    LOCK.lock();
    print_integerln(unsafe { TMP });
    LOCK.unlock();
}
