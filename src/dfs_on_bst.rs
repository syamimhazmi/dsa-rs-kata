use crate::binary_node::BinaryNode;

impl BinaryNode {
    /// searches for a value in the binary search tree
    /// returns true if the value is found
    pub fn contains(&self, needle: i32) -> bool {
        match needle.cmp(&self.value) {
            std::cmp::Ordering::Equal => true,
            std::cmp::Ordering::Less => self
                .left
                .as_ref()
                .map_or(false, |node| node.contains(needle)),
            std::cmp::Ordering::Greater => self
                .right
                .as_ref()
                .map_or(false, |node| node.contains(needle)),
        }
    }

    /// inserts a value into the binary search tree
    /// maintains BST property (left <= parent < right)
    pub fn insert(&mut self, value: i32) {
        match value.cmp(&self.value) {
            std::cmp::Ordering::Less => {
                if let Some(ref mut left) = self.left {
                    left.insert(value);
                } else {
                    self.left = Some(Box::new(BinaryNode::new(value)));
                }
            }
            std::cmp::Ordering::Greater => {
                if let Some(ref mut right) = self.right {
                    right.insert(value);
                } else {
                    self.right = Some(Box::new(BinaryNode::new(value)));
                }
            }
            std::cmp::Ordering::Equal => {}
        }
    }
}

fn dfs(head: Option<&BinaryNode>, needle: i32) -> bool {
    head.map_or(false, |node| node.contains(needle))
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Helper function to create a sample BST
    ///        5
    ///       / \
    ///      3   8
    ///     / \   \
    ///    1   4   10
    fn create_sample_tree() -> BinaryNode {
        let mut root = BinaryNode::new(5);
        root.insert(3);
        root.insert(8);
        root.insert(1);
        root.insert(4);
        root.insert(10);
        root
    }

    #[test]
    fn test_search_existing_root_value() {
        let tree = create_sample_tree();
        assert!(tree.contains(5), "Should find root value 5");
        assert!(dfs(Some(&tree), 5), "Free function should also find 5");
    }

    #[test]
    fn test_search_existing_leaf_values() {
        let tree = create_sample_tree();
        assert!(tree.contains(1), "Should find leaf value 1");
        assert!(tree.contains(4), "Should find leaf value 4");
        assert!(tree.contains(10), "Should find leaf value 10");
    }

    #[test]
    fn test_search_non_existing_values() {
        let tree = create_sample_tree();
        assert!(!tree.contains(0), "Should not find 0");
        assert!(!tree.contains(7), "Should not find 7");
        assert!(!tree.contains(100), "Should not find 100");
        assert!(!tree.contains(-5), "Should not find negative values");
    }

    #[test]
    fn test_search_in_single_node_tree() {
        let single_node = BinaryNode::new(42);
        assert!(
            single_node.contains(42),
            "Should find value in single-node tree"
        );
        assert!(!single_node.contains(0), "Should not find other values");
        assert!(!single_node.contains(100), "Should not find other values");
    }

    #[test]
    fn test_search_empty_tree() {
        // Test the free function with None
        assert!(!dfs(None, 5), "Should return false for empty tree");
        assert!(
            !dfs(None, 0),
            "Should return false for any value in empty tree"
        );
    }

    #[test]
    fn test_complex_tree_search() {
        // Create a more complex tree
        //         50
        //       /    \
        //      30     70
        //     /  \   /  \
        //    20  40 60  80
        //   /
        //  10
        let mut root = BinaryNode::new(50);
        root.insert(30);
        root.insert(70);
        root.insert(20);
        root.insert(40);
        root.insert(60);
        root.insert(80);
        root.insert(10);

        // Debug: Print what the contains method returns
        println!("Testing contains(25): {}", root.contains(25));
        println!("Testing contains(55): {}", root.contains(55));
        println!("Testing contains(75): {}", root.contains(75));

        // Test various paths
        assert!(root.contains(10), "Should find leftmost leaf");
        assert!(root.contains(40), "Should find left subtree right child");
        assert!(root.contains(60), "Should find right subtree left child");
        assert!(root.contains(80), "Should find rightmost value");

        // Test non-existing values between nodes
        assert!(!root.contains(25), "Should not find 25 (between 20 and 30)");
        assert!(!root.contains(55), "Should not find 55 (between 50 and 60)");
        assert!(!root.contains(75), "Should not find 75 (between 70 and 80)");
    }
}
