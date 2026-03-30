// Problem 0654: maximum binary tree

use crate::common::tree_node::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution;

impl Solution {
    pub fn construct_maximum_binary_tree(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void constructMaximumBinaryTree()
    //   TreeNode expected = TreeNode.create(Arrays.asList(6, 3, 5, null, 2, 0, null, null, 1));
    //   assertThat(
    //   new Solution().constructMaximumBinaryTree(new int[] {3, 2, 1, 6, 0, 5}).toString(),
    //   equalTo(expected.toString()));
    #[test]
    fn test_construct_maximum_binary_tree() {
        // TODO: 翻译 Java 测试
    }

    // Java: void constructMaximumBinaryTree2()
    //   TreeNode expected = TreeNode.create(Arrays.asList(3, null, 2, null, 1));
    //   assertThat(
    //   new Solution().constructMaximumBinaryTree(new int[] {3, 2, 1}).toString(),
    //   equalTo(expected.toString()));
    #[test]
    fn test_construct_maximum_binary_tree2() {
        // TODO: 翻译 Java 测试
    }
}
