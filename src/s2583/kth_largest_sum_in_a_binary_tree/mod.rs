// Problem 2583: kth largest sum in a binary tree

use crate::common::tree_node::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution;

impl Solution {
    pub fn kth_largest_level_sum(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i64 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void kthLargestLevelSum()
    //   assertThat(
    //   new Solution()
    //   .kthLargestLevelSum(
    //   TreeNode.create(Arrays.asList(5, 8, 9, 2, 1, 3, 7, 4, 6)), 2),
    //   equalTo(13L));
    #[test]
    fn test_kth_largest_level_sum() {
        // TODO: 翻译 Java 测试
    }

    // Java: void kthLargestLevelSum2()
    //   assertThat(
    //   new Solution().kthLargestLevelSum(TreeNode.create(Arrays.asList(1, 2, null, 3)), 1),
    //   equalTo(3L));
    #[test]
    fn test_kth_largest_level_sum2() {
        // TODO: 翻译 Java 测试
    }
}
