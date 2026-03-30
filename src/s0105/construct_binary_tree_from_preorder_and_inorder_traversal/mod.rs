// Problem 0105: construct binary tree from preorder and inorder traversal

use crate::common::tree_node::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution;

impl Solution {
    pub fn get(key: i32) -> i32 {
        todo!()
    }

    pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void constructBinaryTree()
    //   int[] preorder = {3, 9, 20, 15, 7};
    //   int[] inorder = {9, 3, 15, 20, 7};
    //   TreeNode actual = new Solution().buildTree(preorder, inorder);
    //   assertThat(actual.toString(), equalTo("3,9,20,15,7"));
    #[test]
    fn test_construct_binary_tree() {
        // TODO: 翻译 Java 测试
    }

    // Java: void constructBinaryTree2()
    //   int[] preorder = {-1};
    //   int[] inorder = {-1};
    //   TreeNode actual = new Solution().buildTree(preorder, inorder);
    //   assertThat(actual.toString(), equalTo("-1"));
    #[test]
    fn test_construct_binary_tree2() {
        // TODO: 翻译 Java 测试
    }
}
