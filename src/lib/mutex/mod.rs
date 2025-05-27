use core::sync::atomic::{AtomicBool, Ordering};

use crate::syscall::sys_yield;

/// Mutex trait with basic locking interface
pub trait Mutex {
    fn lock(&self);
    fn unlock(&self);
}

/// Spin-based mutex using busy-wait loop
pub struct SpinLock {
    lock: AtomicBool, // true = locked, false = unlocked
}

impl SpinLock {
    pub const fn new() -> Self {
        Self {
            lock: AtomicBool::new(false),
        }
    }
}

impl Mutex for SpinLock {
    fn lock(&self) {
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
            // Busy-wait spin, do nothing on failure
        }
    }

    fn unlock(&self) {
        // Ordering::Release ensures that all previous memory operations
        // are completed before the lock is released. This guarantees that
        // updates to shared data are visible to other threads after the lock is unlocked.
        self.lock.store(false, Ordering::Release);
    }
}

/// Yielding lock which calls sys_yield() when lock acquisition fails
pub struct YieldLock {
    lock: AtomicBool, // true = locked, false = unlocked
}

impl YieldLock {
    pub const fn new() -> Self {
        Self {
            lock: AtomicBool::new(false),
        }
    }
}

impl Mutex for YieldLock {
    fn lock(&self) {
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
            // Yield the CPU to allow other threads to run
            sys_yield();
        }
    }

    fn unlock(&self) {
        // Ordering::Release ensures that all previous memory operations
        // are completed before the lock is released. This guarantees that
        // updates to shared data are visible to other threads after the lock is unlocked.
        self.lock.store(false, Ordering::Release);
    }
}
