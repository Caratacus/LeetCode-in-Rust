// Problem 0103: binary tree zigzag level order traversal

use crate::common::tree_node::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution;

impl Solution {
    pub fn zigzag_level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void zigzagLevelOrder()
    //   TreeNode root = TreeUtils.constructBinaryTree(Arrays.asList(3, 9, 20, null, null, 15, 7));
    //   assertThat(
    //   new Solution().zigzagLevelOrder(root),
    //   equalTo(ArrayUtils.getLists(new int[][] {{3}, {20, 9}, {15, 7}})));
    #[test]
    fn test_zigzag_level_order() {
        // TODO: 翻译 Java 测试
    }

    // Java: void zigzagLevelOrder2()
    //   TreeNode root = TreeUtils.constructBinaryTree(Arrays.asList(1));
    //   assertThat(
    //   new Solution().zigzagLevelOrder(root),
    //   equalTo(ArrayUtils.getLists(new int[][] {{1}})));
    #[test]
    fn test_zigzag_level_order2() {
        // TODO: 翻译 Java 测试
    }

    // Java: void zigzagLevelOrder3()
    //   assertThat(
    //   new Solution().zigzagLevelOrder(null),
    //   equalTo(ArrayUtils.getLists(new int[][] {})));
    #[test]
    fn test_zigzag_level_order3() {
        // TODO: 翻译 Java 测试
    }
}
