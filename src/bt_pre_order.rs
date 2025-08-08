use crate::binary_node::BinaryNode;

fn walk(curr: Option<&BinaryNode>, path: &mut Vec<i32>) {
    if let Some(node) = curr {
        // recurse
        // pre
        path.push(node.value);
        // recurse
        walk(node.left.as_deref(), path);
        walk(node.right.as_deref(), path);
        // post
    }
}

fn pre_order_search(head: Option<&BinaryNode>) -> Vec<i32> {
    let mut path: Vec<i32> = Vec::new();
    walk(head, &mut path);
    path
}

// NOTE: alternative way withou recursive walk
#[allow(dead_code)]
pub fn pre_order_search_direct(head: Option<&BinaryNode>) -> Vec<i32> {
    match head {
        None => Vec::new(),
        Some(node) => {
            let mut result = vec![node.value];
            result.extend(pre_order_search_direct(node.left.as_deref()));
            result.extend(pre_order_search_direct(node.right.as_deref()));
            result
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{binary_node::BinaryNode, bt_pre_order::pre_order_search};

    #[test]
    fn test_preorder_traversal() {
        // Create a simple binary tree:
        //       1
        //      / \
        //     2   3
        //    / \
        //   4   5

        let tree = Some(Box::new(BinaryNode::with_children(
            1,
            Some(Box::new(BinaryNode::with_children(
                2,
                Some(Box::new(BinaryNode::new(4))),
                Some(Box::new(BinaryNode::new(5))),
            ))),
            Some(Box::new(BinaryNode::new(3))),
        )));

        let result = pre_order_search(tree.as_deref());
        assert_eq!(result, vec![1, 2, 4, 5, 3]);

        // let result2 = pre_order_search_direct(tree.as_deref());
        // assert_eq!(result2, vec![1, 2, 4, 5, 3]);
    }

    #[test]
    fn test_empty_tree() {
        let result = pre_order_search(None);
        assert_eq!(result, Vec::<i32>::new());
    }

    #[test]
    fn test_single_node() {
        let tree = Some(Box::new(BinaryNode::new(42)));
        let result = pre_order_search(tree.as_deref());
        assert_eq!(result, vec![42]);
    }
}
