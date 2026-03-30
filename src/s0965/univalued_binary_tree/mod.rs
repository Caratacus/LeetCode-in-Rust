// Problem 0965: univalued binary tree

use crate::common::tree_node::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution;

impl Solution {
    pub fn is_unival_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void isUnivalTree()
    //   assertThat(
    //   new Solution().isUnivalTree(TreeNode.create(Arrays.asList(1, 1, 1, 1, 1, null, 1))),
    //   equalTo(true));
    #[test]
    fn test_is_unival_tree() {
        // TODO: 翻译 Java 测试
    }

    // Java: void isUnivalTree2()
    //   assertThat(
    //   new Solution().isUnivalTree(TreeNode.create(Arrays.asList(2, 2, 2, 5, 2))),
    //   equalTo(false));
    #[test]
    fn test_is_unival_tree2() {
        // TODO: 翻译 Java 测试
    }
}
