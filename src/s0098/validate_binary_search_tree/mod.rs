// Problem 0098: validate binary search tree

use crate::common::tree_node::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution;

impl Solution {
    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void isValidBST()
    //   TreeNode treeNode1 = new TreeNode(2);
    //   treeNode1.left = new TreeNode(1);
    //   treeNode1.right = new TreeNode(3);
    //   assertThat(new Solution().isValidBST(treeNode1), equalTo(true));
    #[test]
    fn test_is_valid_bst() {
        // TODO: 翻译 Java 测试
    }

    // Java: void isValidBST2()
    //   TreeNode treeNode = new TreeNode(5);
    //   treeNode.left = new TreeNode(1);
    //   treeNode.right = new TreeNode(4);
    //   treeNode.right.left = new TreeNode(3);
    //   treeNode.right.right = new TreeNode(6);
    //   ... (1 more lines)
    #[test]
    fn test_is_valid_bst2() {
        // TODO: 翻译 Java 测试
    }
}
