use std::{cell::RefCell, rc::Rc};

#[derive(Debug)]
struct Node<T> {
    value: T,
    next: Option<Rc<RefCell<Node<T>>>>,
}

#[derive(Debug)]
pub struct Queue<T> {
    pub length: usize,
    head: Option<Rc<RefCell<Node<T>>>>,
    tail: Option<Rc<RefCell<Node<T>>>>,
}

impl<T: std::fmt::Debug> Queue<T> {
    pub fn new() -> Self {
        Self {
            length: 0,
            head: None,
            tail: None,
        }
    }

    pub fn enqueue(&mut self, item: T) {
        let new_node = Rc::new(RefCell::new(Node {
            value: item,
            next: None,
        }));

        self.length += 1;

        match self.tail.take() {
            Some(old_tail) => {
                old_tail.borrow_mut().next = Some(new_node.clone());
                self.tail = Some(new_node);
            }
            None => {
                self.head = Some(new_node.clone());
                self.tail = Some(new_node)
            }
        }
    }

    pub fn dequeue(&mut self) -> Option<T> {
        self.head.take().map(|head| {
            match head.borrow_mut().next.take() {
                Some(new_head) => self.head = Some(new_head),
                None => self.tail = None,
            }

            self.length -= 1;

            Rc::try_unwrap(head).unwrap().into_inner().value
        })
    }

    pub fn peek(&self) -> Option<T>
    where
        T: Clone,
    {
        self.head.as_ref().map(|head| head.borrow().value.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_queue_operations() {
        let mut queue: Queue<i32> = Queue::new();

        // Test initial state
        assert_eq!(queue.length, 0);
        assert_eq!(queue.peek(), None);
        assert_eq!(queue.dequeue(), None);

        // Test enqueue and length tracking
        queue.enqueue(10);
        assert_eq!(queue.length, 1);
        assert_eq!(queue.peek(), Some(10));

        queue.enqueue(20);
        queue.enqueue(30);
        assert_eq!(queue.length, 3);
        assert_eq!(queue.peek(), Some(10));

        // Test FIFO dequeue behavior
        assert_eq!(queue.dequeue(), Some(10));
        assert_eq!(queue.length, 2);
        assert_eq!(queue.peek(), Some(20));

        assert_eq!(queue.dequeue(), Some(20));
        assert_eq!(queue.dequeue(), Some(30));
        assert_eq!(queue.length, 0);
        assert_eq!(queue.peek(), None);
    }

    #[test]
    fn test_empty_queue_operations() {
        let mut queue: Queue<i32> = Queue::new();

        // Test operations on empty queue
        assert_eq!(queue.length, 0);
        assert_eq!(queue.dequeue(), None);
        assert_eq!(queue.peek(), None);

        // Ensure multiple dequeue calls on empty queue don't crash
        assert_eq!(queue.dequeue(), None);
        assert_eq!(queue.dequeue(), None);
        assert_eq!(queue.length, 0);
    }

    #[test]
    fn test_single_element_queue() {
        let mut queue: Queue<&str> = Queue::new();

        // Add single element
        queue.enqueue("hello");
        assert_eq!(queue.length, 1);
        assert_eq!(queue.peek(), Some("hello"));

        // Peek doesn't modify queue
        assert_eq!(queue.peek(), Some("hello"));
        assert_eq!(queue.length, 1);

        // Dequeue single element
        assert_eq!(queue.dequeue(), Some("hello"));
        assert_eq!(queue.length, 0);
        assert_eq!(queue.peek(), None);

        // Queue should be empty now
        assert_eq!(queue.dequeue(), None);
    }

    #[test]
    fn test_fifo_order_with_strings() {
        let mut queue: Queue<&str> = Queue::new();
        let items = vec!["first", "second", "third", "fourth", "fifth"];

        // Enqueue all items
        for item in &items {
            queue.enqueue(*item);
        }
        assert_eq!(queue.length, 5);

        // Dequeue and verify FIFO order
        for (i, expected_item) in items.iter().enumerate() {
            assert_eq!(queue.peek(), Some(*expected_item));
            assert_eq!(queue.dequeue(), Some(*expected_item));
            assert_eq!(queue.length, 4 - i);
        }

        // Queue should be empty
        assert_eq!(queue.length, 0);
        assert_eq!(queue.peek(), None);
    }

    #[test]
    fn test_mixed_operations_and_length_consistency() {
        let mut queue: Queue<i32> = Queue::new();

        // Mix of enqueue and dequeue operations
        queue.enqueue(1);
        queue.enqueue(2);
        assert_eq!(queue.length, 2);

        assert_eq!(queue.dequeue(), Some(1));
        assert_eq!(queue.length, 1);

        queue.enqueue(3);
        queue.enqueue(4);
        assert_eq!(queue.length, 3);
        assert_eq!(queue.peek(), Some(2));

        assert_eq!(queue.dequeue(), Some(2));
        assert_eq!(queue.dequeue(), Some(3));
        assert_eq!(queue.length, 1);

        queue.enqueue(5);
        assert_eq!(queue.length, 2);

        // Final state verification
        assert_eq!(queue.dequeue(), Some(4));
        assert_eq!(queue.dequeue(), Some(5));
        assert_eq!(queue.length, 0);
        assert_eq!(queue.dequeue(), None);
    }
}
