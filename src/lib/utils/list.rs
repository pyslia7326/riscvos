use crate::utils::malloc::free;
use crate::utils::malloc::malloc;
use core::ptr::NonNull;

pub struct ListNode<T> {
    pub value: T,
    pub prev: Option<NonNull<ListNode<T>>>,
    pub next: Option<NonNull<ListNode<T>>>,
}

impl<T: Default> ListNode<T> {
    fn new(value: T) -> Option<NonNull<ListNode<T>>> {
        let size = core::mem::size_of::<ListNode<T>>();
        let raw_ptr = unsafe { malloc(size)? as *mut ListNode<T> };
        unsafe {
            raw_ptr.write(ListNode {
                value,
                prev: None,
                next: None,
            });
            NonNull::new(raw_ptr)
        }
    }
}

pub struct LinkedList<T> {
    pub head: Option<NonNull<ListNode<T>>>,
}

impl<T: Default> LinkedList<T> {
    pub fn new() -> Option<Self> {
        let node_ptr = ListNode::new(T::default())?;
        unsafe {
            (*node_ptr.as_ptr()).prev = Some(node_ptr);
            (*node_ptr.as_ptr()).next = Some(node_ptr);
        }
        Some(Self {
            head: Some(node_ptr),
        })
    }

    pub fn is_empty(&self) -> bool {
        let head = self.head.expect("LinkedList head should not be None");
        unsafe { (*head.as_ptr()).next == Some(head) }
    }

    pub fn push_front(&mut self, value: T) -> Option<NonNull<ListNode<T>>> {
        let head = self.head?;
        let node = ListNode::new(value)?;
        unsafe {
            let next = (*head.as_ptr()).next?;
            (*next.as_ptr()).prev = Some(node);
            (*node.as_ptr()).next = Some(next);
            (*node.as_ptr()).prev = Some(head);
            (*head.as_ptr()).next = Some(node);
        }
        Some(node)
    }

    pub fn push_back(&mut self, value: T) -> Option<NonNull<ListNode<T>>> {
        let head = self.head?;
        let node = ListNode::new(value)?;
        unsafe {
            let prev = (*head.as_ptr()).prev?;
            (*prev.as_ptr()).next = Some(node);
            (*node.as_ptr()).prev = Some(prev);
            (*node.as_ptr()).next = Some(head);
            (*head.as_ptr()).prev = Some(node);
        }
        Some(node)
    }

    pub fn pop_front(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }
        let head = self.head?;
        unsafe {
            let node = (*head.as_ptr()).next?;
            let next = (*node.as_ptr()).next?;
            let prev = (*node.as_ptr()).prev?;
            (*next.as_ptr()).prev = Some(prev);
            (*prev.as_ptr()).next = Some(next);
            let val = core::ptr::read(&(*node.as_ptr()).value);
            free(node.as_ptr() as *mut u8);
            Some(val)
        }
    }

    pub fn pop_back(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }
        let head = self.head?;
        unsafe {
            let node = (*head.as_ptr()).prev?;
            let next = (*node.as_ptr()).next?;
            let prev = (*node.as_ptr()).prev?;
            (*next.as_ptr()).prev = Some(prev);
            (*prev.as_ptr()).next = Some(next);
            let val = core::ptr::read(&(*node.as_ptr()).value);
            free(node.as_ptr() as *mut u8);
            Some(val)
        }
    }

    // iter?
}

pub struct LinkedListIter<'a, T> {
    head: NonNull<ListNode<T>>,
    current: Option<NonNull<ListNode<T>>>,
    _marker: core::marker::PhantomData<&'a T>,
}

impl<T: Default> LinkedList<T> {
    pub fn iter(&self) -> Option<LinkedListIter<'_, T>> {
        let head = self.head?;
        Some(LinkedListIter {
            head,
            current: unsafe { (*head.as_ptr()).next },
            _marker: core::marker::PhantomData,
        })
    }
}

impl<'a, T> Iterator for LinkedListIter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        let current_ptr = self.current?;
        unsafe {
            if current_ptr == self.head {
                return None;
            }
            let node = current_ptr.as_ref();
            self.current = node.next;
            Some(&node.value)
        }
    }
}
