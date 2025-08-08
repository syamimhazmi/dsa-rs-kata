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

    /// extract the minimum value from a tree, removing the node
    fn extract_min(node: &mut Box<BinaryNode>) -> i32 {
        if node.left.is_none() {
            node.value
        } else if node.left.as_ref().unwrap().left.is_none() {
            let min_node = node.left.take().unwrap();
            let min_value = min_node.value;
            node.left = min_node.right;
            min_value
        } else {
            Self::extract_min(node.left.as_mut().unwrap())
        }
    }

    /// deletes a value from the binary search tree
    /// Returns true if the value was found and deleted
    /// Note: This method cannot delete the root node itself as it would need to replace self
    pub fn delete(&mut self, value: i32) -> bool {
        if value == self.value {
            // Cannot delete the root node with this method
            // Use delete_in_place with Option<Box<BinaryNode>> instead
            false
        } else {
            Self::delete_from_child(
                if value < self.value {
                    &mut self.left
                } else {
                    &mut self.right
                },
                value,
            )
        }
    }

    /// Helper function to delete from a child subtree
    fn delete_from_child(node: &mut Option<Box<BinaryNode>>, value: i32) -> bool {
        match node {
            None => false,
            Some(n) => {
                if value == n.value {
                    // Found the node to delete
                    *node = match (n.left.take(), n.right.take()) {
                        (None, None) => None,
                        (Some(left), None) => Some(left),
                        (None, Some(right)) => Some(right),
                        (Some(left), Some(mut right)) => {
                            // Find minimum in right subtree
                            if right.left.is_none() {
                                right.left = Some(left);
                                Some(right)
                            } else {
                                let min_value = Self::extract_min(&mut right);
                                n.value = min_value;
                                n.left = Some(left);
                                n.right = Some(right);
                                return true;
                            }
                        }
                    };
                    true
                } else if value < n.value {
                    Self::delete_from_child(&mut n.left, value)
                } else {
                    Self::delete_from_child(&mut n.right, value)
                }
            }
        }
    }

    /// Alternative delete method that modifies the tree in place
    /// Returns true if the node was found and deleted
    pub fn delete_in_place(node: &mut Option<Box<BinaryNode>>, value: i32) -> bool {
        match node {
            None => false,
            Some(n) => {
                match value.cmp(&n.value) {
                    std::cmp::Ordering::Less => Self::delete_in_place(&mut n.left, value),
                    std::cmp::Ordering::Greater => Self::delete_in_place(&mut n.right, value),
                    std::cmp::Ordering::Equal => {
                        // Found the node to delete
                        *node = match (n.left.take(), n.right.take()) {
                            // Leaf node
                            (None, None) => None,
                            // Only left child
                            (Some(left), None) => Some(left),
                            // Only right child
                            (None, Some(right)) => Some(right),
                            // Two children
                            (Some(left), Some(mut right)) => {
                                // Find and remove the minimum from right subtree
                                if right.left.is_none() {
                                    // Right child is the minimum
                                    right.left = Some(left);
                                    Some(right)
                                } else {
                                    // Find the leftmost node in right subtree
                                    let min_value = Self::extract_min(&mut right);
                                    Some(Box::new(BinaryNode::with_children(
                                        min_value,
                                        Some(left),
                                        Some(right),
                                    )))
                                }
                            }
                        };
                        true
                    }
                }
            }
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

    #[test]
    fn test_delete_leaf_node() {
        // Create tree:
        //       5
        //      / \
        //     3   8
        //    / \
        //   1   4
        let mut root = BinaryNode::new(5);
        root.insert(3);
        root.insert(8);
        root.insert(1);
        root.insert(4);

        // Delete leaf nodes
        let mut tree_opt = Some(Box::new(root.clone()));
        assert!(
            BinaryNode::delete_in_place(&mut tree_opt, 1),
            "Should delete leaf 1"
        );
        assert!(
            !tree_opt.as_ref().unwrap().contains(1),
            "Should not find 1 after deletion"
        );
        assert!(
            tree_opt.as_ref().unwrap().contains(3),
            "Should still find 3"
        );

        assert!(
            BinaryNode::delete_in_place(&mut tree_opt, 4),
            "Should delete leaf 4"
        );
        assert!(
            !tree_opt.as_ref().unwrap().contains(4),
            "Should not find 4 after deletion"
        );
    }

    #[test]
    fn test_delete_node_with_one_child() {
        // Create tree:
        //       5
        //      / \
        //     3   8
        //    /     \
        //   1      10
        let mut root = BinaryNode::new(5);
        root.insert(3);
        root.insert(8);
        root.insert(1);
        root.insert(10);

        // Delete node with one child (3 has only left child, 8 has only right child)
        let mut tree_opt = Some(Box::new(root.clone()));
        assert!(
            BinaryNode::delete_in_place(&mut tree_opt, 8),
            "Should delete 8"
        );
        assert!(
            !tree_opt.as_ref().unwrap().contains(8),
            "Should not find 8 after deletion"
        );
        assert!(
            tree_opt.as_ref().unwrap().contains(10),
            "Should still find 10 (child of deleted 8)"
        );

        assert!(
            BinaryNode::delete_in_place(&mut tree_opt, 3),
            "Should delete 3"
        );
        assert!(
            !tree_opt.as_ref().unwrap().contains(3),
            "Should not find 3 after deletion"
        );
        assert!(
            tree_opt.as_ref().unwrap().contains(1),
            "Should still find 1 (child of deleted 3)"
        );
    }

    #[test]
    fn test_delete_node_with_two_children() {
        // Create tree:
        //         50
        //       /    \
        //      30     70
        //     /  \   /  \
        //    20  40 60  80
        let mut root = BinaryNode::new(50);
        root.insert(30);
        root.insert(70);
        root.insert(20);
        root.insert(40);
        root.insert(60);
        root.insert(80);

        // Delete node with two children (30 has 20 and 40 as children)
        let mut tree_opt = Some(Box::new(root.clone()));
        assert!(
            BinaryNode::delete_in_place(&mut tree_opt, 30),
            "Should delete 30"
        );
        assert!(
            !tree_opt.as_ref().unwrap().contains(30),
            "Should not find 30 after deletion"
        );
        assert!(
            tree_opt.as_ref().unwrap().contains(20),
            "Should still find 20"
        );
        assert!(
            tree_opt.as_ref().unwrap().contains(40),
            "Should still find 40"
        );

        // Verify BST property is maintained
        // The inorder successor (40) should have replaced 30
        // Tree should still be a valid BST
        assert!(
            tree_opt.as_ref().unwrap().contains(50),
            "Root should still be 50"
        );
        assert!(
            tree_opt.as_ref().unwrap().contains(70),
            "Should still find 70"
        );
    }

    #[test]
    fn test_delete_root_node() {
        // Test deleting root with two children
        //       5
        //      / \
        //     3   8
        //    /   / \
        //   1   7   9
        let mut root = BinaryNode::new(5);
        root.insert(3);
        root.insert(8);
        root.insert(1);
        root.insert(7);
        root.insert(9);

        let mut tree_opt = Some(Box::new(root));
        assert!(
            BinaryNode::delete_in_place(&mut tree_opt, 5),
            "Should delete root 5"
        );
        assert!(
            !tree_opt.as_ref().unwrap().contains(5),
            "Should not find 5 after deletion"
        );

        // All other nodes should still be present
        assert!(
            tree_opt.as_ref().unwrap().contains(3),
            "Should still find 3"
        );
        assert!(
            tree_opt.as_ref().unwrap().contains(8),
            "Should still find 8"
        );
        assert!(
            tree_opt.as_ref().unwrap().contains(1),
            "Should still find 1"
        );
        assert!(
            tree_opt.as_ref().unwrap().contains(7),
            "Should still find 7"
        );
        assert!(
            tree_opt.as_ref().unwrap().contains(9),
            "Should still find 9"
        );
    }

    #[test]
    fn test_delete_non_existent_node() {
        let mut root = BinaryNode::new(5);
        root.insert(3);
        root.insert(8);

        let mut tree_opt = Some(Box::new(root.clone()));
        assert!(
            !BinaryNode::delete_in_place(&mut tree_opt, 10),
            "Should return false for non-existent node"
        );
        assert!(
            !BinaryNode::delete_in_place(&mut tree_opt, 1),
            "Should return false for non-existent node"
        );

        // Tree should remain unchanged
        assert!(
            tree_opt.as_ref().unwrap().contains(5),
            "Should still find 5"
        );
        assert!(
            tree_opt.as_ref().unwrap().contains(3),
            "Should still find 3"
        );
        assert!(
            tree_opt.as_ref().unwrap().contains(8),
            "Should still find 8"
        );
    }
}
