// Problem 0107: binary tree level order traversal ii

use crate::common::tree_node::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution;

impl Solution {
    pub fn level_order_bottom(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        todo!()
    }

    pub fn get_order(root: Option<Rc<RefCell<TreeNode>>>, level: i32) -> () {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void levelOrderBottom()
    //   TreeNode root = TreeUtils.constructBinaryTree(Arrays.asList(3, 9, 20, null, null, 15, 7));
    //   assertThat(
    //   new Solution().levelOrderBottom(root),
    //   equalTo(ArrayUtils.getLists(new int[][] {{15, 7}, {9, 20}, {3}})));
    #[test]
    fn test_level_order_bottom() {
        // TODO: 翻译 Java 测试
    }

    // Java: void levelOrderBottom2()
    //   TreeNode root = TreeUtils.constructBinaryTree(Arrays.asList(1));
    //   assertThat(
    //   new Solution().levelOrderBottom(root),
    //   equalTo(ArrayUtils.getLists(new int[][] {{1}})));
    #[test]
    fn test_level_order_bottom2() {
        // TODO: 翻译 Java 测试
    }

    // Java: void levelOrderBottom3()
    //   assertThat(
    //   new Solution().levelOrderBottom(null),
    //   equalTo(ArrayUtils.getLists(new int[][] {})));
    #[test]
    fn test_level_order_bottom3() {
        // TODO: 翻译 Java 测试
    }
}
