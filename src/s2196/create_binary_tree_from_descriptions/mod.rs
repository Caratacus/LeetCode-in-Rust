// Problem 2196: create binary tree from descriptions

use crate::common::tree_node::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution;

impl Solution {
    pub fn create_binary_tree(descriptions: Vec<Vec<i32>>) -> Option<Rc<RefCell<TreeNode>>> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void createBinaryTree()
    //   assertThat(
    //   new Solution()
    //   .createBinaryTree(
    //   new int[][] {
    //   {20, 15, 1}, {20, 17, 0}, {50, 20, 1}, {50, 80, 0}, {80, 19, 1}
    //   ... (3 more lines)
    #[test]
    fn test_create_binary_tree() {
        // TODO: 翻译 Java 测试
    }

    // Java: void createBinaryTree2()
    //   assertThat(
    //   new Solution()
    //   .createBinaryTree(new int[][] {{1, 2, 1}, {2, 3, 0}, {3, 4, 1}})
    //   .toString(),
    //   equalTo("1,2,null,3,4,null,null"));
    #[test]
    fn test_create_binary_tree2() {
        // TODO: 翻译 Java 测试
    }
}
