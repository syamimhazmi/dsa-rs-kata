#[derive(Debug)]
struct Node<T> {
    value: T,
    previous: Option<Box<Node<T>>>,
}

#[derive(Debug)]
pub struct Stack<T> {
    length: usize,
    head: Option<Box<Node<T>>>,
}

impl<T: std::fmt::Debug> Stack<T> {
    pub fn new() -> Self {
        Stack {
            head: None,
            length: 0,
        }
    }

    pub fn push(&mut self, item: T) {
        let node = Box::new(Node {
            value: item,
            previous: self.head.take(),
        });

        self.head = Some(node);
        self.length += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.previous;
            self.length = self.length.saturating_sub(1);
            node.value
        })
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Test Suite 1: Basic Stack Operations
    #[test]
    fn test_basic_stack_operations() {
        let mut stack = Stack::new();

        // Test new stack is empty
        assert_eq!(stack.length, 0);
        assert!(stack.pop().is_none());

        // Test push operation
        stack.push(10);
        assert_eq!(stack.length, 1);

        stack.push(20);
        stack.push(30);
        assert_eq!(stack.length, 3);

        // Test pop operation (LIFO - Last In, First Out)
        assert_eq!(stack.pop(), Some(30));
        assert_eq!(stack.length, 2);

        assert_eq!(stack.pop(), Some(20));
        assert_eq!(stack.length, 1);

        assert_eq!(stack.pop(), Some(10));
        assert_eq!(stack.length, 0);

        // Test pop from empty stack
        assert_eq!(stack.pop(), None);
        assert_eq!(stack.length, 0);
    }

    // Test Suite 2: Peek Functionality
    #[test]
    fn test_peek_operations() {
        let mut stack = Stack::new();

        // Test peek on empty stack
        assert!(stack.peek().is_none());

        // Test peek with single element
        stack.push(42);
        assert_eq!(stack.peek(), Some(&42));
        assert_eq!(stack.length, 1); // Peek shouldn't change length

        // Test peek with multiple elements
        stack.push(100);
        stack.push(200);
        assert_eq!(stack.peek(), Some(&200)); // Should return top element
        assert_eq!(stack.length, 3);

        // Test peek doesn't affect pop
        assert_eq!(stack.pop(), Some(200));
        assert_eq!(stack.peek(), Some(&100));

        // Test peek after popping all elements
        stack.pop();
        stack.pop();
        assert!(stack.peek().is_none());
    }

    // Test Suite 3: String Type Operations
    #[test]
    fn test_string_stack_operations() {
        let mut stack = Stack::new();

        // Test with String values
        stack.push("first".to_string());
        stack.push("second".to_string());
        stack.push("third".to_string());

        assert_eq!(stack.length, 3);
        assert_eq!(stack.peek(), Some(&"third".to_string()));

        // Test popping strings
        assert_eq!(stack.pop(), Some("third".to_string()));
        assert_eq!(stack.pop(), Some("second".to_string()));
        assert_eq!(stack.pop(), Some("first".to_string()));

        assert_eq!(stack.length, 0);
        assert!(stack.pop().is_none());

        // Test with string slices
        let mut str_stack = Stack::new();
        str_stack.push("hello");
        str_stack.push("world");

        assert_eq!(str_stack.peek(), Some(&"world"));
        assert_eq!(str_stack.pop(), Some("world"));
        assert_eq!(str_stack.pop(), Some("hello"));
    }

    // Test Suite 4: Complex Data Types and Edge Cases
    #[test]
    fn test_complex_types_and_edge_cases() {
        // Test with tuples
        let mut tuple_stack = Stack::new();
        tuple_stack.push((1, "a"));
        tuple_stack.push((2, "b"));
        tuple_stack.push((3, "c"));

        assert_eq!(tuple_stack.pop(), Some((3, "c")));
        assert_eq!(tuple_stack.peek(), Some(&(2, "b")));

        // Test with vectors
        let mut vec_stack = Stack::new();
        vec_stack.push(vec![1, 2, 3]);
        vec_stack.push(vec![4, 5, 6]);

        assert_eq!(vec_stack.pop(), Some(vec![4, 5, 6]));
        assert_eq!(vec_stack.peek(), Some(&vec![1, 2, 3]));

        // Test length tracking with zero values
        let mut zero_stack = Stack::new();
        zero_stack.push(0);
        zero_stack.push(0);
        zero_stack.push(0);

        assert_eq!(zero_stack.length, 3);
        assert_eq!(zero_stack.pop(), Some(0));
        assert_eq!(zero_stack.length, 2);

        // Test saturating_sub behavior (length should never go below 0)
        let mut edge_stack = Stack::new();
        edge_stack.push(1);
        edge_stack.pop();
        edge_stack.pop(); // Pop from empty stack
        assert_eq!(edge_stack.length, 0);
    }

    // Test Suite 5: Stress Test and Mixed Operations
    #[test]
    fn test_stress_and_mixed_operations() {
        let mut stack = Stack::new();

        // Push many elements
        for i in 0..1000 {
            stack.push(i);
        }
        assert_eq!(stack.length, 1000);
        assert_eq!(stack.peek(), Some(&999));

        // Pop half of them
        for i in (500..1000).rev() {
            assert_eq!(stack.pop(), Some(i));
        }
        assert_eq!(stack.length, 500);
        assert_eq!(stack.peek(), Some(&499));

        // Mixed push and pop operations
        stack.push(1000);
        stack.push(1001);
        assert_eq!(stack.length, 502);

        assert_eq!(stack.pop(), Some(1001));
        assert_eq!(stack.pop(), Some(1000));
        assert_eq!(stack.pop(), Some(499));
        assert_eq!(stack.length, 499);

        // Clear the entire stack
        while stack.pop().is_some() {
            // Keep popping until empty
        }
        assert_eq!(stack.length, 0);
        assert!(stack.peek().is_none());

        // Test operations on cleared stack
        stack.push(9999);
        assert_eq!(stack.length, 1);
        assert_eq!(stack.peek(), Some(&9999));
        assert_eq!(stack.pop(), Some(9999));
        assert_eq!(stack.length, 0);
    }
}
