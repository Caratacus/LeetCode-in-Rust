// Problem 0865: smallest subtree with all the deepest nodes

use crate::common::tree_node::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution;

impl Solution {
    pub fn subtree_with_all_deepest(
        root: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void subtreeWithAllDeepest()
    //   assertThat(
    //   new Solution()
    //   .subtreeWithAllDeepest(
    //   TreeUtils.constructBinaryTree(
    //   Arrays.asList(3, 5, 1, 6, 2, 0, 8, null, null, 7, 4)))
    //   ... (2 more lines)
    #[test]
    fn test_subtree_with_all_deepest() {
        // TODO: 翻译 Java 测试
    }

    // Java: void subtreeWithAllDeepest2()
    //   assertThat(
    //   new Solution()
    //   .subtreeWithAllDeepest(TreeUtils.constructBinaryTree(Arrays.asList(1)))
    //   .toString(),
    //   equalTo("1"));
    #[test]
    fn test_subtree_with_all_deepest2() {
        // TODO: 翻译 Java 测试
    }

    // Java: void subtreeWithAllDeepest3()
    //   assertThat(
    //   new Solution()
    //   .subtreeWithAllDeepest(
    //   TreeUtils.constructBinaryTree(Arrays.asList(0, 1, 3, null, 2)))
    //   .toString(),
    //   ... (1 more lines)
    #[test]
    fn test_subtree_with_all_deepest3() {
        // TODO: 翻译 Java 测试
    }
}
