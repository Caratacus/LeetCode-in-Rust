// Problem 0230: kth smallest element in a bst

use crate::common::tree_node::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution;

impl Solution {
    pub fn kth_smallest(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void kthSmallest()
    //   TreeNode root = TreeNode.create(Arrays.asList(3, 1, 4, null, 2));
    //   assertThat(new Solution().kthSmallest(root, 1), equalTo(1));
    #[test]
    fn test_kth_smallest() {
        // TODO: 翻译 Java 测试
    }

    // Java: void kthSmallest2()
    //   TreeNode root = TreeNode.create(Arrays.asList(5, 3, 6, 2, 4, null, null, 1));
    //   assertThat(new Solution().kthSmallest(root, 3), equalTo(3));
    #[test]
    fn test_kth_smallest2() {
        // TODO: 翻译 Java 测试
    }
}
