use crate::utils::malloc::free;
use crate::utils::malloc::malloc;
use core::ptr::NonNull;
use core::sync::atomic::{AtomicUsize, Ordering};

struct ArcInner<T> {
    ref_cnt: AtomicUsize,
    value: T,
}

pub struct Arc<T> {
    ptr: NonNull<ArcInner<T>>,
}

impl<T> Arc<T> {
    pub fn new(value: T) -> Option<Self> {
        let size = core::mem::size_of::<ArcInner<T>>();
        let raw = unsafe { malloc(size)? as *mut ArcInner<T> };
        unsafe {
            raw.write(ArcInner {
                ref_cnt: AtomicUsize::new(1),
                value,
            });
            Some(Self {
                ptr: NonNull::new_unchecked(raw),
            })
        }
    }

    pub fn clone(&self) -> Self {
        let inner = unsafe { self.ptr.as_ref() };
        inner.ref_cnt.fetch_add(1, Ordering::Relaxed);
        Self {
            ptr: self.ptr.clone(),
        }
    }

    pub fn get_ref(&self) -> &T {
        unsafe { &self.ptr.as_ref().value }
    }

    pub fn ptr_eq(a1: &Arc<T>, a2: &Arc<T>) -> bool {
        a1.ptr == a2.ptr
    }
}

impl<T> Drop for Arc<T> {
    fn drop(&mut self) {
        let inner = unsafe { self.ptr.as_ref() };
        if inner.ref_cnt.fetch_sub(1, Ordering::Release) == 1 {
            core::sync::atomic::fence(Ordering::Acquire);
            unsafe {
                core::ptr::drop_in_place(&mut (*self.ptr.as_ptr()).value);
                free(self.ptr.as_ptr() as *mut u8);
            }
        }
    }
}
