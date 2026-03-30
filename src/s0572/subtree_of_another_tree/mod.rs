// Problem 0572: subtree of another tree

use crate::common::tree_node::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution;

impl Solution {
    pub fn is_subtree(
        root: Option<Rc<RefCell<TreeNode>>>,
        sub_root: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void isSubtree()
    //   TreeNode treeNode = TreeUtils.constructBinaryTree(Arrays.asList(3, 4, 5, 1, 2));
    //   TreeNode subTree = TreeUtils.constructBinaryTree(Arrays.asList(4, 1, 2));
    //   assertThat(new Solution().isSubtree(treeNode, subTree), equalTo(true));
    #[test]
    fn test_is_subtree() {
        // TODO: 翻译 Java 测试
    }

    // Java: void isSubtree2()
    //   TreeNode treeNode =
    //   TreeUtils.constructBinaryTree(
    //   Arrays.asList(3, 4, 5, 1, 2, null, null, null, null, 0));
    //   TreeNode subTree = TreeUtils.constructBinaryTree(Arrays.asList(4, 1, 2));
    //   assertThat(new Solution().isSubtree(treeNode, subTree), equalTo(false));
    #[test]
    fn test_is_subtree2() {
        // TODO: 翻译 Java 测试
    }
}
