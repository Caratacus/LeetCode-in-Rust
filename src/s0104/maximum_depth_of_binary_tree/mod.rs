// Problem 0104: maximum depth of binary tree

use crate::common::tree_node::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution;

impl Solution {
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void maxDepthBinaryTree()
    //   TreeNode root = TreeUtils.constructBinaryTree(Arrays.asList(3, 9, 20, null, null, 15, 7));
    //   assertThat(new Solution().maxDepth(root), equalTo(3));
    #[test]
    fn test_max_depth_binary_tree() {
        // TODO: 翻译 Java 测试
    }

    // Java: void maxDepthBinaryTree2()
    //   TreeNode root = TreeUtils.constructBinaryTree(Arrays.asList(1, null, 2));
    //   assertThat(new Solution().maxDepth(root), equalTo(2));
    #[test]
    fn test_max_depth_binary_tree2() {
        // TODO: 翻译 Java 测试
    }
}
