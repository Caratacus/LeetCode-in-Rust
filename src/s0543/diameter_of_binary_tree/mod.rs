// Problem 0543: diameter of binary tree

use crate::common::tree_node::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution;

impl Solution {
    pub fn diameter_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void diameterOfBinaryTree()
    //   assertThat(
    //   new Solution().diameterOfBinaryTree(TreeNode.create(Arrays.asList(1, 2, 3, 4, 5))),
    //   equalTo(3));
    #[test]
    fn test_diameter_of_binary_tree() {
        // TODO: 翻译 Java 测试
    }

    // Java: void diameterOfBinaryTree2()
    //   assertThat(
    //   new Solution().diameterOfBinaryTree(TreeNode.create(Arrays.asList(1, 2))),
    //   equalTo(1));
    #[test]
    fn test_diameter_of_binary_tree2() {
        // TODO: 翻译 Java 测试
    }
}
