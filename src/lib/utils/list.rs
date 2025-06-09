use crate::mutex::Mutex;
use crate::mutex::YieldLock;
use crate::utils::rc::Arc;
use core::ops::Drop;

pub struct ListNode<T> {
    pub value: Option<T>,
    pub prev: Option<Arc<Mutex<ListNode<T>, YieldLock>>>,
    pub next: Option<Arc<Mutex<ListNode<T>, YieldLock>>>,
}

pub struct LinkedList<T> {
    pub head: Option<Arc<Mutex<ListNode<T>, YieldLock>>>,
}

impl<T> ListNode<T> {
    fn new(value: Option<T>) -> Option<Arc<Mutex<Self, YieldLock>>> {
        Arc::new(Mutex::new(
            YieldLock::new(),
            ListNode {
                value,
                prev: None,
                next: None,
            },
        ))
    }

    fn init_list_head(
        node: Option<Arc<Mutex<Self, YieldLock>>>,
    ) -> Option<Arc<Mutex<Self, YieldLock>>> {
        let node = node?;
        let prev = node.clone();
        let next = node.clone();
        {
            let mut node_guard = node.get_ref().lock();
            node_guard.prev = Some(prev);
            node_guard.next = Some(next);
        }
        Some(node)
    }
}

impl<T> Drop for ListNode<T> {
    fn drop(&mut self) {
        self.prev.take();
        self.next.take();
    }
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        let head = ListNode::new(None);
        let head = ListNode::init_list_head(head);
        Self { head }
    }

    pub fn is_empty(&self) -> bool {
        let head_arc = self
            .head
            .as_ref()
            .expect("LinkedList head should not be None");
        let head_guard = head_arc.get_ref().lock();
        let next_arc = head_guard
            .next
            .as_ref()
            .expect("ListNode next should always point to some node");
        Arc::ptr_eq(head_arc, next_arc)
    }

    pub fn push_back(&self, value: T) -> Option<Arc<Mutex<ListNode<T>, YieldLock>>> {
        let next_arc = self.head.as_ref()?;
        let prev_arc = {
            let next_guard = next_arc.get_ref().lock();
            next_guard.prev.as_ref()?.clone()
        };
        let node_arc = ListNode::new(Some(value))?;
        {
            let mut node_guard = node_arc.get_ref().lock();
            node_guard.prev = Some(prev_arc.clone());
            node_guard.next = Some(next_arc.clone());
        }
        {
            let mut prev_guard = prev_arc.get_ref().lock();
            prev_guard.next = Some(node_arc.clone());
        }
        {
            let mut next_guard = next_arc.get_ref().lock();
            next_guard.prev = Some(node_arc.clone());
        }
        Some(node_arc)
    }

    pub fn push_front(&self, value: T) -> Option<Arc<Mutex<ListNode<T>, YieldLock>>> {
        let prev_arc = self.head.as_ref()?;
        let next_arc = {
            let prev_guard = prev_arc.get_ref().lock();
            prev_guard.next.as_ref()?.clone()
        };
        let node_arc = ListNode::new(Some(value))?;
        {
            let mut node_guard = node_arc.get_ref().lock();
            node_guard.prev = Some(prev_arc.clone());
            node_guard.next = Some(next_arc.clone());
        }
        {
            let mut prev_guard = prev_arc.get_ref().lock();
            prev_guard.next = Some(node_arc.clone());
        }
        {
            let mut next_guard = next_arc.get_ref().lock();
            next_guard.prev = Some(node_arc.clone());
        }
        Some(node_arc)
    }

    fn remove_node_safe(
        node_to_remove: Arc<Mutex<ListNode<T>, YieldLock>>,
    ) -> Option<Arc<Mutex<ListNode<T>, YieldLock>>> {
        let (prev_arc, next_arc) = {
            let mut node_guard = node_to_remove.get_ref().lock();
            let prev = node_guard.prev.take()?;
            let next = node_guard.next.take()?;
            (prev, next)
        };
        {
            let mut prev_guard = prev_arc.get_ref().lock();
            prev_guard.next = Some(next_arc.clone());
        }
        {
            let mut next_guard = next_arc.get_ref().lock();
            next_guard.prev = Some(prev_arc.clone());
        }
        Some(node_to_remove)
    }

    pub fn pop_back(&self) -> Option<Arc<Mutex<ListNode<T>, YieldLock>>> {
        if self.is_empty() {
            return None;
        }
        let next_arc = self.head.as_ref()?;
        let pop_arc = {
            let next_guard = next_arc.get_ref().lock();
            next_guard.prev.as_ref()?.clone()
        };
        Self::remove_node_safe(pop_arc)
    }

    pub fn pop_front(&self) -> Option<Arc<Mutex<ListNode<T>, YieldLock>>> {
        if self.is_empty() {
            return None;
        }
        let prev_arc = self.head.as_ref()?;
        let pop_arc = {
            let prev_guard = prev_arc.get_ref().lock();
            prev_guard.next.as_ref()?.clone()
        };
        Self::remove_node_safe(pop_arc)
    }
}

impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        while let Some(_) = self.pop_front() {}
        self.head.take();
    }
}

pub struct LinkedListIter<'a, T> {
    head: &'a Arc<Mutex<ListNode<T>, YieldLock>>,
    current: Arc<Mutex<ListNode<T>, YieldLock>>,
    _marker: core::marker::PhantomData<&'a T>,
}

impl<T> LinkedList<T> {
    pub fn iter(&self) -> Option<LinkedListIter<'_, T>> {
        let head = self.head.as_ref()?;
        let next = {
            let head_guard = head.get_ref().lock();
            head_guard.next.as_ref()?.clone()
        };
        Some(LinkedListIter {
            head,
            current: next,
            _marker: core::marker::PhantomData,
        })
    }
}

impl<'a, T: Clone> Iterator for LinkedListIter<'a, T> {
    type Item = Arc<Mutex<ListNode<T>, YieldLock>>;
    fn next(&mut self) -> Option<Self::Item> {
        if Arc::ptr_eq(self.head, &self.current) {
            return None;
        }
        let current_node = self.current.clone();
        self.current = {
            let current_guard = current_node.get_ref().lock();
            current_guard.next.as_ref()?.clone()
        };
        Some(current_node)
    }
}
