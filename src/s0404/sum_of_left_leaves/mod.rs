// Problem 0404: sum of left leaves

use crate::common::tree_node::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution;

impl Solution {
    pub fn sum_of_left_leaves(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void sumOfLeftLeaves()
    //   assertThat(
    //   new Solution()
    //   .sumOfLeftLeaves(
    //   TreeNode.create(Arrays.asList(3, 9, 20, null, null, 15, 7))),
    //   equalTo(24));
    #[test]
    fn test_sum_of_left_leaves() {
        // TODO: 翻译 Java 测试
    }

    // Java: void sumOfLeftLeaves2()
    //   assertThat(new Solution().sumOfLeftLeaves(TreeNode.create(Arrays.asList(1))), equalTo(0));
    #[test]
    fn test_sum_of_left_leaves2() {
        // TODO: 翻译 Java 测试
    }
}
