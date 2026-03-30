// Problem 0106: construct binary tree from inorder and postorder traversal

use crate::common::tree_node::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution;

impl Solution {
    pub fn build_tree(inorder: Vec<i32>, postorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void constructBinaryTree()
    //   int[] inorder = {9, 3, 15, 20, 7};
    //   int[] postorder = {9, 15, 7, 20, 3};
    //   TreeNode actual = new Solution().buildTree(inorder, postorder);
    //   assertThat(actual.toString(), equalTo("3,9,20,15,7"));
    #[test]
    fn test_construct_binary_tree() {
        // TODO: 翻译 Java 测试
    }

    // Java: void constructBinaryTree2()
    //   int[] inorder = {-1};
    //   int[] postorder = {-1};
    //   TreeNode actual = new Solution().buildTree(inorder, postorder);
    //   assertThat(actual.toString(), equalTo("-1"));
    #[test]
    fn test_construct_binary_tree2() {
        // TODO: 翻译 Java 测试
    }
}
