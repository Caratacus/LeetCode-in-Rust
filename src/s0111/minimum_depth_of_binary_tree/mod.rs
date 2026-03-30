// Problem 0111: minimum depth of binary tree

use crate::common::tree_node::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution;

impl Solution {
    pub fn min_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void minDepth()
    //   TreeNode root = TreeUtils.constructBinaryTree(Arrays.asList(3, 9, 20, null, null, 15, 7));
    //   assertThat(new Solution().minDepth(root), equalTo(2));
    #[test]
    fn test_min_depth() {
        // TODO: 翻译 Java 测试
    }

    // Java: void minDepth2()
    //   TreeNode root =
    //   TreeUtils.constructBinaryTree(Arrays.asList(2, null, 3, null, 4, null, 5, null, 6));
    //   assertThat(new Solution().minDepth(root), equalTo(5));
    #[test]
    fn test_min_depth2() {
        // TODO: 翻译 Java 测试
    }
}
