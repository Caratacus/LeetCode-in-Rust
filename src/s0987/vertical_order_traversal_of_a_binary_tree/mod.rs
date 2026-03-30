// Problem 0987: vertical order traversal of a binary tree

use crate::common::tree_node::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution;

impl Solution {
    pub fn vertical_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void verticalTraversal()
    //   assertThat(
    //   new Solution()
    //   .verticalTraversal(
    //   TreeUtils.constructBinaryTree(
    //   Arrays.asList(3, 9, 20, null, null, 15, 7))),
    //   ... (1 more lines)
    #[test]
    fn test_vertical_traversal() {
        // TODO: 翻译 Java 测试
    }

    // Java: void verticalTraversal2()
    //   assertThat(
    //   new Solution()
    //   .verticalTraversal(
    //   TreeUtils.constructBinaryTree(Arrays.asList(1, 2, 3, 4, 5, 6, 7))),
    //   equalTo(ArrayUtils.getLists(new int[][] {{4}, {2}, {1, 5, 6}, {3}, {7}})));
    #[test]
    fn test_vertical_traversal2() {
        // TODO: 翻译 Java 测试
    }
}
