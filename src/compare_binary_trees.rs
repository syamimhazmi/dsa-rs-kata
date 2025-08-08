use crate::binary_node::BinaryNode;

fn compare(a: Option<&BinaryNode>, b: Option<&BinaryNode>) -> bool {
    match (a, b) {
        // structural check
        (None, None) => true,
        // structural check
        (None, Some(_)) | (Some(_), None) => false,
        // structural and value check
        (Some(a), Some(b)) => {
            a.value == b.value
                && compare(a.left.as_deref(), b.left.as_deref())
                && compare(a.right.as_deref(), b.right.as_deref())
        }
    }

    // or you can just use one line; since we are using PartialEq trait.
    // a == b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compare_approaches() {
        let tree1 = BinaryNode::with_children(
            5,
            Some(Box::new(BinaryNode::new(3))),
            Some(Box::new(BinaryNode::new(7))),
        );

        let tree2 = BinaryNode::with_children(
            5,
            Some(Box::new(BinaryNode::new(3))),
            Some(Box::new(BinaryNode::new(7))),
        );

        assert!(tree1 == tree2);
        assert!(compare(Some(&tree1), Some(&tree2)));
    }

    #[test]
    fn test_compare_empty_trees() {
        assert!(compare(None, None));
    }

    #[test]
    fn test_compare_one_empty_one_not() {
        let node = BinaryNode::new(5);

        assert!(!compare(Some(&node), None));
        assert!(!compare(None, Some(&node)));
    }

    #[test]
    fn test_compare_single_nodes() {
        let node1 = BinaryNode::new(5);
        let node2 = BinaryNode::new(5);
        let node3 = BinaryNode::new(3);

        assert!(compare(Some(&node1), Some(&node2)));
        assert!(!compare(Some(&node1), Some(&node3)));

        // More idiomatic direct comparison
        assert!(node1 == node2);
        assert!(node1 != node3);
    }

    #[test]
    fn test_compare_complex_trees() {
        // Tree 1:     5
        //           /   \
        //          3     7
        //         / \   /
        //        1   4 6

        let tree1 = BinaryNode::with_children(
            5,
            Some(Box::new(BinaryNode::with_children(
                3,
                Some(Box::new(BinaryNode::new(1))),
                Some(Box::new(BinaryNode::new(4))),
            ))),
            Some(Box::new(BinaryNode::with_children(
                7,
                Some(Box::new(BinaryNode::new(6))),
                None,
            ))),
        );

        // Tree 2: Same structure and values
        let tree2 = BinaryNode::with_children(
            5,
            Some(Box::new(BinaryNode::with_children(
                3,
                Some(Box::new(BinaryNode::new(1))),
                Some(Box::new(BinaryNode::new(4))),
            ))),
            Some(Box::new(BinaryNode::with_children(
                7,
                Some(Box::new(BinaryNode::new(6))),
                None,
            ))),
        );

        // Tree 3: Different structure
        let tree3 = BinaryNode::with_children(
            5,
            Some(Box::new(BinaryNode::new(3))),
            Some(Box::new(BinaryNode::new(7))),
        );

        assert!(compare(Some(&tree1), Some(&tree2)));
        assert!(!compare(Some(&tree1), Some(&tree3)));

        assert!(tree1 == tree2);
        assert!(tree1 != tree3);
    }
}
