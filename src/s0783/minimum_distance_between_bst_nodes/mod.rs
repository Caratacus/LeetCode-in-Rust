// Problem 0783: minimum distance between bst nodes

use crate::common::tree_node::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution;

impl Solution {
    pub fn min_diff_in_bst(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void minDiffInBST()
    //   assertThat(
    //   new Solution()
    //   .minDiffInBST(
    //   TreeUtils.constructBinaryTree(
    //   Arrays.asList(4, 2, 6, 1, 3, null, null))),
    //   ... (1 more lines)
    #[test]
    fn test_min_diff_in_bst() {
        // TODO: 翻译 Java 测试
    }

    // Java: void minDiffInBST2()
    //   assertThat(
    //   new Solution()
    //   .minDiffInBST(
    //   TreeUtils.constructBinaryTree(
    //   Arrays.asList(1, 0, 48, null, null, 12, 49))),
    //   ... (1 more lines)
    #[test]
    fn test_min_diff_in_bst2() {
        // TODO: 翻译 Java 测试
    }
}
