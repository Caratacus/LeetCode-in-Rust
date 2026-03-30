// Problem 2265: count nodes equal to average of subtree

use crate::common::tree_node::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution;

impl Solution {
    pub fn average_of_subtree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void averageOfSubtree()
    //   TreeNode root = TreeUtils.constructBinaryTree(Arrays.asList(4, 8, 5, 0, 1, null, 6));
    //   assertThat(new Solution().averageOfSubtree(root), equalTo(5));
    #[test]
    fn test_average_of_subtree() {
        // TODO: 翻译 Java 测试
    }

    // Java: void averageOfSubtree2()
    //   TreeNode root = TreeUtils.constructBinaryTree(Arrays.asList(1));
    //   assertThat(new Solution().averageOfSubtree(root), equalTo(1));
    #[test]
    fn test_average_of_subtree2() {
        // TODO: 翻译 Java 测试
    }
}
