use std::collections::VecDeque;

use crate::binary_node::BinaryNode;

/// breadth first search
fn bfs(head: Option<&BinaryNode>, needle: i32) -> bool {
    let Some(head_node) = head else {
        return false;
    };

    let mut q: VecDeque<&BinaryNode> = VecDeque::from([head_node]);

    while let Some(curr) = q.pop_front() {
        // search
        if curr.value == needle {
            return true;
        }

        // check children in queue if the value of left and right is not empty
        if let Some(left) = curr.left.as_deref() {
            q.push_back(left);
        }

        if let Some(right) = curr.left.as_deref() {
            q.push_back(right);
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    fn new_node(value: i32) -> Box<BinaryNode> {
        Box::new(BinaryNode {
            value,
            left: None,
            right: None,
        })
    }

    fn new_node_with_children(
        value: i32,
        left: Option<Box<BinaryNode>>,
        right: Option<Box<BinaryNode>>,
    ) -> Box<BinaryNode> {
        Box::new(BinaryNode { value, left, right })
    }

    #[test]
    fn test_bfs_empty_tree() {
        assert!(!bfs(None, 42));
    }

    #[test]
    fn test_bfs_single_node_found() {
        let root = new_node(42);
        assert!(bfs(Some(&root), 42));
    }

    #[test]
    fn test_bfs_single_node_not_found() {
        let root = new_node(42);
        assert!(!bfs(Some(&root), 99));
    }

    #[test]
    fn test_bfs_complex_tree() {
        // Tree structure:
        //       4
        //      / \
        //     2   6
        //    / \ / \
        //   1  3 5  7

        let root = new_node_with_children(
            4,
            Some(new_node_with_children(
                2,
                Some(new_node(1)),
                Some(new_node(3)),
            )),
            Some(new_node_with_children(
                6,
                Some(new_node(5)),
                Some(new_node(7)),
            )),
        );

        assert!(bfs(Some(&root), 4)); // Root
        assert!(bfs(Some(&root), 2)); // Level 1
        assert!(!bfs(Some(&root), 6)); // Level 1
        assert!(bfs(Some(&root), 1)); // Level 2
        assert!(!bfs(Some(&root), 7)); // Level 2
        assert!(!bfs(Some(&root), 99)); // Not in tree
    }
}
