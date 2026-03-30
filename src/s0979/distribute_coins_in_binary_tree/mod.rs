// Problem 0979: distribute coins in binary tree

use crate::common::tree_node::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution;

impl Solution {
    pub fn distribute_coins(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void distributeCoins()
    //   assertThat(
    //   new Solution()
    //   .distributeCoins(TreeUtils.constructBinaryTree(Arrays.asList(3, 0, 0))),
    //   equalTo(2));
    #[test]
    fn test_distribute_coins() {
        // TODO: 翻译 Java 测试
    }

    // Java: void distributeCoins2()
    //   assertThat(
    //   new Solution()
    //   .distributeCoins(TreeUtils.constructBinaryTree(Arrays.asList(0, 3, 0))),
    //   equalTo(3));
    #[test]
    fn test_distribute_coins2() {
        // TODO: 翻译 Java 测试
    }

    // Java: void distributeCoins3()
    //   assertThat(
    //   new Solution()
    //   .distributeCoins(
    //   TreeUtils.constructBinaryTree(Arrays.asList(1, 0, 0, null, 3))),
    //   equalTo(4));
    #[test]
    fn test_distribute_coins3() {
        // TODO: 翻译 Java 测试
    }
}
