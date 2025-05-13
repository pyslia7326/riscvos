use core::sync::atomic::{AtomicBool, Ordering};

use crate::syscall::sys_yield;

pub struct Mutex {
    lock: AtomicBool, // true = locked, false = unlocked
}

impl Mutex {
    pub const fn new() -> Self {
        Self {
            lock: AtomicBool::new(false),
        }
    }

    pub fn lock(&self) {
        // Attempts to acquire the lock using compare_exchange.
        // Ordering::Acquire ensures that all subsequent memory operations
        // will be observed after the lock is successfully acquired.
        // Ordering::Relaxed is sufficient on failure, as no synchronization is needed
        // when the lock is not acquired.
        while self
            .lock
            .compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed)
            .is_err()
        {
            sys_yield();
        }
    }

    pub fn unlock(&self) {
        // Ordering::Release ensures that all previous memory operations
        // are completed before the lock is released. This guarantees that
        // updates to shared data are visible to other threads after the lock is unlocked.
        self.lock.store(false, Ordering::Release);
    }
}
