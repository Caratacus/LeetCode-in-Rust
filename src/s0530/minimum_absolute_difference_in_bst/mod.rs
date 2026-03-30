// Problem 0530: minimum absolute difference in bst

use crate::common::tree_node::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution;

impl Solution {
    pub fn get_minimum_difference(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void getMinimumDifference()
    //   TreeNode treeNode = TreeUtils.constructBinaryTree(Arrays.asList(4, 2, 6, 1, 3));
    //   assertThat(new Solution().getMinimumDifference(treeNode), equalTo(1));
    #[test]
    fn test_get_minimum_difference() {
        // TODO: 翻译 Java 测试
    }

    // Java: void getMinimumDifference2()
    //   TreeNode treeNode =
    //   TreeUtils.constructBinaryTree(Arrays.asList(1, 0, 48, null, null, 12, 49));
    //   assertThat(new Solution().getMinimumDifference(treeNode), equalTo(1));
    #[test]
    fn test_get_minimum_difference2() {
        // TODO: 翻译 Java 测试
    }
}
