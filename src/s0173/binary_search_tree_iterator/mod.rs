// Problem 0173: binary search tree iterator

use crate::common::tree_node::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub struct BSTIterator {
    root: Option<Rc<RefCell<TreeNode>>>,
}

impl BSTIterator {
    pub fn new(root: Option<Rc<RefCell<TreeNode>>>) -> Self {
        todo!()
    }

    pub fn next(&mut self) -> i32 {
        todo!()
    }

    pub fn has_next(&mut self) -> bool {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void iteratorBST()
    //   TreeNode left = new TreeNode(3);
    //   TreeNode right = new TreeNode(15, new TreeNode(9), new TreeNode(20));
    //   TreeNode root = new TreeNode(7, left, right);
    //   BSTIterator iterator = new BSTIterator(root);
    //   assertThat(iterator.next(), equalTo(3));
    //   ... (8 more lines)
    #[test]
    fn test_iterator_bst() {
        // TODO: 翻译 Java 测试
    }
}
