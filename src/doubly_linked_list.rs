use std::{cell::RefCell, rc::Rc};

type NodeRef<T> = Rc<RefCell<Node<T>>>;

#[derive(Debug)]
struct Node<T> {
    value: T,
    next: Option<NodeRef<T>>,
    prev: Option<NodeRef<T>>,
}

impl<T> Node<T> {
    fn new(value: T) -> NodeRef<T> {
        Rc::new(RefCell::new(Node {
            value,
            next: None,
            prev: None,
        }))
    }
}

#[derive(Debug)]
struct DoublyLinkedList<T> {
    head: Option<NodeRef<T>>,
    tail: Option<NodeRef<T>>,
    length: usize,
}

impl<T> DoublyLinkedList<T> {
    pub fn new() -> Self {
        DoublyLinkedList {
            head: None,
            tail: None,
            length: 0,
        }
    }

    pub fn len(&self) -> usize {
        self.length
    }

    pub fn is_empty(&self) -> bool {
        self.length == 0
    }

    pub fn prepend(&mut self, item: T) {
        let new_node = Node::new(item);

        match self.head.take() {
            Some(current_head) => {
                // point old head to new node
                // NewNode <- OldHead
                current_head.borrow_mut().prev = Some(new_node.clone());
                // point new node to old head
                // NewNode -> OldHead
                new_node.borrow_mut().next = Some(current_head);

                // set current head to new node
                self.head = Some(new_node);
            }
            None => {
                self.head = Some(new_node.clone());
                self.tail = Some(new_node);
            }
        }

        self.length += 1;
    }

    pub fn insert_at(&mut self, item: T, idx: usize) {
        if idx > self.length {
            panic!("index out of bounds");
        }

        if idx == self.length {
            self.append(item);
            return;
        }

        if idx == 0 {
            self.prepend(item);
            return;
        }

        let mut current = self.head.clone();

        for _ in 0..idx {
            if let Some(curr) = current {
                current = curr.borrow_mut().next.clone();
            }
        }

        let new_node = Node::new(item);

        if let Some(curr) = current {
            let prev_node = curr.borrow_mut().prev.clone();

            new_node.borrow_mut().next = Some(curr.clone());
            new_node.borrow_mut().prev = prev_node.clone();

            if let Some(prev) = prev_node {
                prev.borrow_mut().next = Some(new_node.clone());
            }

            curr.borrow_mut().prev = Some(new_node);
        }

        self.length += 1;
    }

    pub fn append(&mut self, item: T) {
        let new_node = Node::new(item);

        match self.tail.take() {
            Some(curr_tail) => {
                // point current tail to new node
                // CurrTail -> NewNode
                curr_tail.borrow_mut().next = Some(new_node.clone());
                // point new node to current tail
                // CurrTail <- NewNode
                new_node.borrow_mut().prev = Some(curr_tail);

                // point tail to new node
                self.tail = Some(new_node);
            }
            None => {
                self.head = Some(new_node.clone());
                self.tail = Some(new_node);
            }
        }

        self.length += 1;
    }

    pub fn remove(&mut self, item: T) -> Option<T>
    where
        T: PartialEq + Clone, // we use PartialEq trait to compare current borrow value that is in
                              // binary so that it can compare with type: T
                              // PartialEq trait will do the rest
    {
        let mut current = self.head.clone();
        let mut idx = 0;

        while let Some(curr) = current {
            if curr.borrow().value == item {
                return self.remove_at(idx);
            }

            current = curr.borrow().next.clone();

            idx += 1;
        }

        None
    }

    pub fn get(&self, idx: usize) -> Option<T>
    where
        T: Clone,
    {
        if idx >= self.length {
            return None;
        }

        let mut current = self.head.clone();

        for _ in 0..idx {
            if let Some(curr) = current {
                current = curr.borrow().next.clone();
            } else {
                return None;
            }
        }

        current.map(|node| node.borrow().value.clone())
    }

    pub fn remove_at(&mut self, idx: usize) -> Option<T>
    where
        T: Clone,
    {
        if idx >= self.length {
            return None;
        }

        if self.length == 1 {
            let value = self.head.as_ref()?.borrow().value.clone();
            self.head = None;
            self.tail = None;
            self.length = 0;
            return Some(value);
        }

        let mut current = self.head.clone();

        // navigate to the position
        // find the node for the idx position
        for _ in 0..idx {
            if let Some(curr) = current {
                current = curr.borrow().next.clone();
            } else {
                return None;
            }
        }

        // we find the node to be removed
        if let Some(node_to_remove) = current {
            let value = node_to_remove.borrow().value.clone();
            let prev_node = node_to_remove.borrow().prev.clone();
            let next_node = node_to_remove.borrow().next.clone();

            // update previous node's next pointer to next node from node that need to be removed
            // from: A -> C -> B, to: A -> B
            if let Some(ref prev) = prev_node {
                prev.borrow_mut().next = next_node.clone();
            } else {
                // assign head to the next node
                self.head = next_node.clone()
            }

            // update next node's prev pointer to previous  from node that need to be removed
            // from: A <- C <-, to: A <- B
            if let Some(ref next) = next_node {
                next.borrow_mut().prev = prev_node.clone();
            } else {
                // assign tail to the previous node
                self.tail = prev_node.clone();
            }

            self.length -= 1;
            Some(value)
        } else {
            None
        }
    }

    pub fn iter(&self) -> DoublyLinkedListIter<T> {
        DoublyLinkedListIter {
            current: self.head.clone(),
        }
    }
}

pub struct DoublyLinkedListIter<T> {
    current: Option<NodeRef<T>>,
}

impl<T: Clone> Iterator for DoublyLinkedListIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(current) = self.current.take() {
            let value = current.borrow().value.clone();
            self.current = current.borrow().next.clone();
            Some(value)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_append_and_get() {
        let mut list = DoublyLinkedList::new();
        list.append(1);
        list.append(2);
        list.append(3);

        assert_eq!(list.get(0), Some(1));
        assert_eq!(list.get(1), Some(2));
        assert_eq!(list.get(2), Some(3));
        assert_eq!(list.get(3), None);
    }

    #[test]
    fn test_insert_at() {
        let mut list = DoublyLinkedList::new();
        list.append(1);
        list.append(3);
        list.insert_at(2, 1);

        assert_eq!(list.get(0), Some(1));
        assert_eq!(list.get(1), Some(2));
        assert_eq!(list.get(2), Some(3));
    }

    #[test]
    fn test_remove_at() {
        let mut list = DoublyLinkedList::new();
        list.append(1);
        list.append(2);
        list.append(3);

        assert_eq!(list.remove_at(1), Some(2));
        assert_eq!(list.get(0), Some(1));
        assert_eq!(list.get(1), Some(3));
        assert_eq!(list.len(), 2);
    }

    #[test]
    fn test_remove() {
        let mut list = DoublyLinkedList::new();
        list.append(1);
        list.append(2);
        list.append(3);

        assert_eq!(list.remove(2), Some(2));
        assert_eq!(list.get(0), Some(1));
        assert_eq!(list.get(1), Some(3));
        assert_eq!(list.len(), 2);
    }

    #[test]
    fn test_empy_list_operations() {
        let mut list: DoublyLinkedList<i32> = DoublyLinkedList::new();

        assert_eq!(list.len(), 0);
        assert!(list.is_empty());
        assert_eq!(list.get(0), None);
        assert_eq!(list.remove_at(0), None);
        assert_eq!(list.remove(42), None);
    }

    #[test]
    fn test_single_element_list() {
        let mut list = DoublyLinkedList::new();
        list.append(42);

        assert_eq!(list.len(), 1);
        assert!(!list.is_empty());
        assert_eq!(list.get(0), Some(42));
        assert_eq!(list.get(1), None);

        assert_eq!(list.remove_at(0), Some(42));
        assert_eq!(list.len(), 0);
        assert!(list.is_empty());
        assert_eq!(list.get(0), None);
    }

    #[test]
    fn test_prepend_operations() {
        let mut list = DoublyLinkedList::new();

        list.prepend(1);
        assert_eq!(list.get(0), Some(1));
        assert_eq!(list.len(), 1);

        list.prepend(0);
        assert_eq!(list.get(0), Some(0));
        assert_eq!(list.get(1), Some(1));
        assert_eq!(list.len(), 2);
    }

    #[test]
    fn test_boundary_insertions() {
        let mut list = DoublyLinkedList::new();

        list.insert_at(10, 0);
        assert_eq!(list.get(0), Some(10));
        assert_eq!(list.len(), 1);

        list.insert_at(20, 1);
        assert_eq!(list.get(1), Some(20));
        assert_eq!(list.len(), 2);

        list.insert_at(5, 0);
        assert_eq!(list.get(0), Some(5));
        assert_eq!(list.get(1), Some(10));
        assert_eq!(list.get(2), Some(20));
        assert_eq!(list.len(), 3);
    }

    #[test]
    #[should_panic(expected = "index out of bounds")]
    fn test_insert_at_invalid_index() {
        let mut list = DoublyLinkedList::new();
        list.append(1);
        list.insert_at(99, 5);
    }

    #[test]
    fn test_remove_at_boundaries() {
        let mut list = DoublyLinkedList::new();
        list.append(1);
        list.append(2);
        list.append(3);

        assert_eq!(list.remove_at(2), Some(3));
        assert_eq!(list.len(), 2);

        assert_eq!(list.remove_at(0), Some(1));
        assert_eq!(list.get(0), Some(2));
        assert_eq!(list.len(), 1);

        assert_eq!(list.remove_at(0), Some(2));
        assert_eq!(list.len(), 0);
        assert!(list.is_empty());
    }

    #[test]
    fn test_remove_nonexistent_item() {
        let mut list = DoublyLinkedList::new();
        list.append(1);
        list.append(2);
        list.append(3);

        assert_eq!(list.remove(99), None);
        assert_eq!(list.len(), 3);

        let mut empty_list: DoublyLinkedList<i32> = DoublyLinkedList::new();
        assert_eq!(empty_list.remove(42), None);
    }

    #[test]
    fn test_out_of_bounds_access() {
        let mut list = DoublyLinkedList::new();
        list.append(1);
        list.append(2);

        assert_eq!(list.get(2), None);
        assert_eq!(list.get(100), None);

        assert_eq!(list.remove_at(2), None);
        assert_eq!(list.remove_at(100), None);
        assert_eq!(list.len(), 2);
    }

    #[test]
    fn test_iterator_on_empty_list() {
        let list: DoublyLinkedList<i32> = DoublyLinkedList::new();
        let mut iter = list.iter();
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_iterator_single_element() {
        let mut list = DoublyLinkedList::new();
        list.append(42);

        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(42));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_large_list_operations() {
        let mut list = DoublyLinkedList::new();

        for i in 0..1000 {
            list.append(i);
        }

        assert_eq!(list.len(), 1000);
        assert_eq!(list.get(0), Some(0));
        assert_eq!(list.get(999), Some(999));
        assert_eq!(list.get(1000), None);

        assert_eq!(list.remove_at(500), Some(500));
        assert_eq!(list.len(), 999);
        assert_eq!(list.get(500), Some(501));
    }

    #[test]
    fn test_alternating_operations() {
        let mut list = DoublyLinkedList::new();

        list.append(2);
        list.prepend(1);
        list.append(3);
        list.prepend(0);

        assert_eq!(list.len(), 4);
        for i in 0..4 {
            assert_eq!(list.get(i), Some(i as i32));
        }

        assert_eq!(list.remove_at(1), Some(1));
        assert_eq!(list.remove(3), Some(3));
        assert_eq!(list.len(), 2);
        assert_eq!(list.get(0), Some(0));
        assert_eq!(list.get(1), Some(2));
    }
}
