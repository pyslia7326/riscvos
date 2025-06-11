use crate::mutex::Lock;
use crate::mutex::YieldLock;

const HEAP_SIZE: usize = 32 * 1024;
static HEAP: [u8; HEAP_SIZE] = [0; HEAP_SIZE];
static mut PROGRAM_BREAK: usize = 0;
static HEAP_LOCK: YieldLock = YieldLock::new();
const ALIGNMENT: usize = 8;

pub unsafe fn malloc(nbytes: usize) -> Option<*mut u8> {
    unsafe {
        HEAP_LOCK.lock();
        let current_program_break_addr = HEAP.as_ptr() as usize + PROGRAM_BREAK;
        let remainder = current_program_break_addr % ALIGNMENT;
        let padding_bytes = if remainder == 0 {
            0
        } else {
            ALIGNMENT - remainder
        };
        let aligned_program_break_offset = PROGRAM_BREAK + padding_bytes;
        if aligned_program_break_offset + nbytes > HEAP_SIZE {
            HEAP_LOCK.unlock();
            return None;
        }
        let ptr = HEAP.as_ptr().add(aligned_program_break_offset);
        PROGRAM_BREAK = aligned_program_break_offset + nbytes;
        HEAP_LOCK.unlock();
        Some(ptr as *mut u8)
    }
}

pub unsafe fn free(_ptr: *mut u8) {
    // do nothing now
}
// TODO (freelist) pub unsafe fn malloc(nbytes: usize) -> Option<*mut u8>;
// TODO pub unsafe fn free(ptr: *mut u8);
