use crate::binary_node::BinaryNode;

fn walk(curr: Option<&BinaryNode>, path: &mut Vec<i32>) {
    if let Some(node) = curr {
        // recurse
        // pre
        // recurse
        walk(node.left.as_deref(), path);
        walk(node.right.as_deref(), path);

        // post
        path.push(node.value);
    }
}

fn post_order_search(head: Option<&BinaryNode>) -> Vec<i32> {
    let mut path: Vec<i32> = Vec::new();
    walk(head, &mut path);
    path
}

#[cfg(test)]
mod tests {
    use super::*;

    // Helper function to create a new node
    fn new_node(value: i32) -> Box<BinaryNode> {
        Box::new(BinaryNode {
            value,
            left: None,
            right: None,
        })
    }

    // Helper function to create a node with children
    fn new_node_with_children(
        value: i32,
        left: Option<Box<BinaryNode>>,
        right: Option<Box<BinaryNode>>,
    ) -> Box<BinaryNode> {
        Box::new(BinaryNode { value, left, right })
    }

    #[test]
    fn test_post_order_empty_tree() {
        // Test Case 1: Empty tree (post-order)
        let result = post_order_search(None);
        assert_eq!(result, Vec::<i32>::new());
        println!("✓ Post-order empty tree test passed");
    }

    #[test]
    fn test_post_order_single_node() {
        // Test Case 2: Single node tree (post-order)
        let root = new_node(42);
        let result = post_order_search(Some(&root));
        assert_eq!(result, vec![42]);
        println!("✓ Post-order single node test passed");
    }

    #[test]
    fn test_post_order_complex_tree() {
        // Test Case 3: Complex binary tree (post-order)
        // Tree structure:
        //       4
        //      / \
        //     2   6
        //    / \ / \
        //   1  3 5  7
        //
        // Expected post-order: [1, 3, 2, 5, 7, 6, 4]
        // (Left → Right → Root for each subtree)

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

        let result = post_order_search(Some(&root));
        assert_eq!(result, vec![1, 3, 2, 5, 7, 6, 4]);
        println!("✓ Post-order complex tree test passed");
    }

    #[test]
    fn test_post_order_unbalanced_tree() {
        // Bonus Test Case 4: Unbalanced tree (post-order)
        // Tree structure:
        //     3
        //    /
        //   2
        //  /
        // 1
        //
        // Expected post-order: [1, 2, 3]

        let root = new_node_with_children(
            3,
            Some(new_node_with_children(2, Some(new_node(1)), None)),
            None,
        );

        let result = post_order_search(Some(&root));
        assert_eq!(result, vec![1, 2, 3]);
        println!("✓ Post-order unbalanced tree test passed");
    }
}
