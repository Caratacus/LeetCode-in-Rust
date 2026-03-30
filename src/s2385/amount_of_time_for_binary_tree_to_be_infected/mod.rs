// Problem 2385: amount of time for binary tree to be infected

use crate::common::tree_node::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution;

impl Solution {
    pub fn amount_of_time(root: Option<Rc<RefCell<TreeNode>>>, start: i32) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void amountOfTime()
    //   assertThat(
    //   new Solution()
    //   .amountOfTime(
    //   TreeNode.create(Arrays.asList(1, 5, 3, null, 4, 10, 6, 9, 2)), 3),
    //   equalTo(4));
    #[test]
    fn test_amount_of_time() {
        // TODO: 翻译 Java 测试
    }

    // Java: void amountOfTime2()
    //   assertThat(new Solution().amountOfTime(TreeNode.create(Arrays.asList(1)), 1), equalTo(0));
    #[test]
    fn test_amount_of_time2() {
        // TODO: 翻译 Java 测试
    }

    // Java: void amountOfTime3()
    //   assertThat(
    //   new Solution()
    //   .amountOfTime(
    //   TreeNode.create(Arrays.asList(1, 2, null, 3, null, 4, null, 5)), 4),
    //   equalTo(3));
    #[test]
    fn test_amount_of_time3() {
        // TODO: 翻译 Java 测试
    }
}
