// Problem 3319: k th largest perfect subtree size in binary tree

use crate::common::tree_node::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution;

impl Solution {
    pub fn kth_largest_perfect_subtree(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void kthLargestPerfectSubtree()
    //   assertThat(
    //   new Solution()
    //   .kthLargestPerfectSubtree(
    //   TreeNode.create(
    //   Arrays.asList(5, 3, 6, 5, 2, 5, 7, 1, 8, null, null, 6, 8)),
    //   ... (2 more lines)
    #[test]
    fn test_kth_largest_perfect_subtree() {
        // TODO: 翻译 Java 测试
    }

    // Java: void kthLargestPerfectSubtree2()
    //   assertThat(
    //   new Solution()
    //   .kthLargestPerfectSubtree(
    //   TreeNode.create(Arrays.asList(1, 2, 3, 4, 5, 6, 7)), 1),
    //   equalTo(7));
    #[test]
    fn test_kth_largest_perfect_subtree2() {
        // TODO: 翻译 Java 测试
    }

    // Java: void kthLargestPerfectSubtree3()
    //   assertThat(
    //   new Solution()
    //   .kthLargestPerfectSubtree(
    //   TreeNode.create(Arrays.asList(1, 2, 3, null, 4)), 3),
    //   equalTo(-1));
    #[test]
    fn test_kth_largest_perfect_subtree3() {
        // TODO: 翻译 Java 测试
    }
}
