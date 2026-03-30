// Problem 0938: range sum of bst

use crate::common::tree_node::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution;

impl Solution {
    pub fn range_sum_bst(root: Option<Rc<RefCell<TreeNode>>>, low: i32, high: i32) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void rangeSumBST()
    //   TreeNode treeNode = TreeNode.create(Arrays.asList(10, 5, 15, 3, 7, null, 18));
    //   assertThat(new Solution().rangeSumBST(treeNode, 7, 15), equalTo(32));
    #[test]
    fn test_range_sum_bst() {
        // TODO: 翻译 Java 测试
    }

    // Java: void rangeSumBST2()
    //   TreeNode treeNode = TreeNode.create(Arrays.asList(10, 5, 15, 3, 7, 13, 18, 1, null, 6));
    //   assertThat(new Solution().rangeSumBST(treeNode, 6, 10), equalTo(23));
    #[test]
    fn test_range_sum_bst2() {
        // TODO: 翻译 Java 测试
    }
}
