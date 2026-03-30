// Problem 0998: maximum binary tree ii

use crate::common::tree_node::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution;

impl Solution {
    pub fn insert_into_max_tree(
        root: Option<Rc<RefCell<TreeNode>>>,
        val: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void insertIntoMaxTree()
    //   assertThat(
    //   new Solution()
    //   .insertIntoMaxTree(
    //   TreeUtils.constructBinaryTree(
    //   Arrays.asList(4, 1, 3, null, null, 2)),
    //   ... (3 more lines)
    #[test]
    fn test_insert_into_max_tree() {
        // TODO: 翻译 Java 测试
    }

    // Java: void insertIntoMaxTree2()
    //   assertThat(
    //   new Solution()
    //   .insertIntoMaxTree(
    //   TreeUtils.constructBinaryTree(Arrays.asList(5, 2, 4, null, 1)), 3)
    //   .toString(),
    //   ... (1 more lines)
    #[test]
    fn test_insert_into_max_tree2() {
        // TODO: 翻译 Java 测试
    }

    // Java: void insertIntoMaxTree3()
    //   assertThat(
    //   new Solution()
    //   .insertIntoMaxTree(
    //   TreeUtils.constructBinaryTree(Arrays.asList(5, 2, 3, null, 1)), 4)
    //   .toString(),
    //   ... (1 more lines)
    #[test]
    fn test_insert_into_max_tree3() {
        // TODO: 翻译 Java 测试
    }
}
