// Problem 2415: reverse odd levels of binary tree

use crate::common::tree_node::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution;

impl Solution {
    pub fn reverse_odd_levels(
        root: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void reverseOddLevels()
    //   assertThat(
    //   new Solution()
    //   .reverseOddLevels(TreeNode.create(Arrays.asList(2, 3, 5, 8, 13, 21, 34)))
    //   .toString(),
    //   equalTo("2,5,8,13,3,21,34"));
    #[test]
    fn test_reverse_odd_levels() {
        // TODO: 翻译 Java 测试
    }

    // Java: void reverseOddLevels2()
    //   assertThat(
    //   new Solution()
    //   .reverseOddLevels(TreeNode.create(Arrays.asList(7, 13, 11)))
    //   .toString(),
    //   equalTo("7,11,13"));
    #[test]
    fn test_reverse_odd_levels2() {
        // TODO: 翻译 Java 测试
    }
}
