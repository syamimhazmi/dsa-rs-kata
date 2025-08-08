#[derive(Debug, Clone, PartialEq)]
pub struct BinaryNode {
    pub value: i32,
    pub left: Option<Box<BinaryNode>>,
    pub right: Option<Box<BinaryNode>>,
}

impl BinaryNode {
    pub fn new(value: i32) -> Self {
        BinaryNode {
            value,
            left: None,
            right: None,
        }
    }

    pub fn with_children(
        value: i32,
        left: Option<Box<BinaryNode>>,
        right: Option<Box<BinaryNode>>,
    ) -> Self {
        BinaryNode { value, left, right }
    }
}
