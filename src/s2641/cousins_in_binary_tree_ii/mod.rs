// Problem 2641: cousins in binary tree ii

use crate::common::tree_node::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution;

impl Solution {
    pub fn replace_value_in_tree(
        root: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void replaceValueInTree()
    //   assertThat(
    //   new Solution()
    //   .replaceValueInTree(TreeNode.create(Arrays.asList(5, 4, 9, 1, 10, null, 7)))
    //   .toString(),
    //   equalTo(TreeNode.create(Arrays.asList(0, 0, 0, 7, 7, null, 11)).toString()));
    #[test]
    fn test_replace_value_in_tree() {
        // TODO: 翻译 Java 测试
    }

    // Java: void replaceValueInTree2()
    //   assertThat(
    //   new Solution()
    //   .replaceValueInTree(TreeNode.create(Arrays.asList(3, 1, 2)))
    //   .toString(),
    //   equalTo(TreeNode.create(Arrays.asList(0, 0, 0)).toString()));
    #[test]
    fn test_replace_value_in_tree2() {
        // TODO: 翻译 Java 测试
    }
}
