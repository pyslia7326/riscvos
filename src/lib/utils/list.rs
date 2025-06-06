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
        let mut node = ListNode::new(T::default())?;
        unsafe {
            (*node.as_mut()).prev = Some(node);
            (*node.as_mut()).next = Some(node);
        }
        Some(Self {
            head: Some(node),
        })
    }

    pub fn is_empty(&self) -> bool {
        let head = self.head.expect("LinkedList head should not be None");
        unsafe { (*head.as_ref()).next == Some(head) }
    }

    pub fn push_front(&mut self, value: T) -> Option<NonNull<ListNode<T>>> {
        let mut head = self.head?;
        let mut node = ListNode::new(value)?;
        unsafe {
            let mut next = (*head.as_ref()).next?;
            (*next.as_mut()).prev = Some(node);
            (*node.as_mut()).next = Some(next);
            (*node.as_mut()).prev = Some(head);
            (*head.as_mut()).next = Some(node);
        }
        Some(node)
    }

    pub fn push_back(&mut self, value: T) -> Option<NonNull<ListNode<T>>> {
        let mut head = self.head?;
        let mut node = ListNode::new(value)?;
        unsafe {
            let mut prev = (*head.as_ref()).prev?;
            (*prev.as_mut()).next = Some(node);
            (*node.as_mut()).prev = Some(prev);
            (*node.as_mut()).next = Some(head);
            (*head.as_mut()).prev = Some(node);
        }
        Some(node)
    }

    pub fn pop_front(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }
        let head = self.head?;
        unsafe {
            let node = (*head.as_ref()).next?;
            let mut next = (*node.as_ref()).next?;
            let mut prev = (*node.as_ref()).prev?;
            (*next.as_mut()).prev = Some(prev);
            (*prev.as_mut()).next = Some(next);
            let val = core::ptr::read(&(*node.as_ref()).value);
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
            let node = (*head.as_ref()).prev?;
            let mut next = (*node.as_ref()).next?;
            let mut prev = (*node.as_ref()).prev?;
            (*next.as_mut()).prev = Some(prev);
            (*prev.as_mut()).next = Some(next);
            let val = core::ptr::read(&(*node.as_ref()).value);
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
            current: unsafe { (*head.as_ref()).next },
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
