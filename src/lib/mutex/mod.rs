use crate::syscall::sys_yield;
use core::cell::UnsafeCell;
use core::ops::{Deref, DerefMut};
use core::sync::atomic::{AtomicBool, Ordering};

/// Lock trait with basic locking interface
pub trait Lock {
    fn lock(&self);
    fn unlock(&self);
}

/// Spin-based lock using busy-wait loop
pub struct SpinLock {
    spin_lock: AtomicBool, // true = locked, false = unlocked
}

impl SpinLock {
    pub const fn new() -> Self {
        Self {
            spin_lock: AtomicBool::new(false),
        }
    }
}

impl Lock for SpinLock {
    fn lock(&self) {
        // Attempts to acquire the lock using compare_exchange.
        // Ordering::Acquire ensures that all subsequent memory operations
        // will be observed after the lock is successfully acquired.
        // Ordering::Relaxed is sufficient on failure, as no synchronization is needed
        // when the lock is not acquired.
        while self
            .spin_lock
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
        self.spin_lock.store(false, Ordering::Release);
    }
}

/// Yielding lock which calls sys_yield() when lock acquisition fails
pub struct YieldLock {
    yield_lock: AtomicBool, // true = locked, false = unlocked
}

impl YieldLock {
    pub const fn new() -> Self {
        Self {
            yield_lock: AtomicBool::new(false),
        }
    }
}

impl Lock for YieldLock {
    fn lock(&self) {
        // Attempts to acquire the lock using compare_exchange.
        // Ordering::Acquire ensures that all subsequent memory operations
        // will be observed after the lock is successfully acquired.
        // Ordering::Relaxed is sufficient on failure, as no synchronization is needed
        // when the lock is not acquired.
        while self
            .yield_lock
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
        self.yield_lock.store(false, Ordering::Release);
    }
}

pub struct Mutex<T, L: Lock> {
    pub lock: L,
    pub data: UnsafeCell<T>,
}

pub struct MutexGuard<'a, T, L: Lock> {
    mutex: &'a Mutex<T, L>,
}

unsafe impl<T: Send, L: Lock + Send> Send for Mutex<T, L> {}
unsafe impl<T: Send, L: Lock + Send + Sync> Sync for Mutex<T, L> {}

impl<T, L: Lock> Mutex<T, L> {
    pub fn new(lock: L, value: T) -> Self {
        Self {
            lock,
            data: UnsafeCell::new(value),
        }
    }

    pub fn lock(&self) -> MutexGuard<'_, T, L> {
        self.lock.lock();
        MutexGuard { mutex: self }
    }
}

impl<'a, T, L: Lock> Deref for MutexGuard<'a, T, L> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.mutex.data.get() }
    }
}

impl<'a, T, L: Lock> DerefMut for MutexGuard<'a, T, L> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.mutex.data.get() }
    }
}

impl<'a, T, L: Lock> Drop for MutexGuard<'a, T, L> {
    fn drop(&mut self) {
        self.mutex.lock.unlock();
    }
}
