use crate::mutex::Lock;
use crate::mutex::YieldLock;

const HEAP_SIZE: usize = 8 * 1024;
static HEAP: [u8; HEAP_SIZE] = [0; HEAP_SIZE];
static mut PROGRAM_BREAK: usize = 0;
static HEAP_LOCK: YieldLock = YieldLock::new();

pub unsafe fn malloc(nbytes: usize) -> Option<*mut u8> {
    unsafe {
        HEAP_LOCK.lock();
        if PROGRAM_BREAK + nbytes > HEAP_SIZE {
            HEAP_LOCK.unlock();
            return None;
        }
        let ptr = HEAP.as_ptr().add(PROGRAM_BREAK);
        PROGRAM_BREAK += nbytes;
        HEAP_LOCK.unlock();
        Some(ptr as *mut u8)
    }
}

pub unsafe fn free(_ptr: *mut u8) {
    // do nothing now
}
// TODO (freelist) pub unsafe fn malloc(nbytes: usize) -> Option<*mut u8>;
// TODO pub unsafe fn free(ptr: *mut u8);
