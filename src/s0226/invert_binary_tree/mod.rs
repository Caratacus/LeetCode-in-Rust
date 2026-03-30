// Problem 0226: invert binary tree

use crate::common::tree_node::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution;

impl Solution {
    pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void invertTree()
    //   TreeNode root = TreeUtils.constructBinaryTree(Arrays.asList(4, 2, 7, 1, 3, 6, 9));
    //   assertThat(new Solution().invertTree(root).toString(), equalTo("4,7,9,6,2,3,1"));
    #[test]
    fn test_invert_tree() {
        // TODO: 翻译 Java 测试
    }

    // Java: void invertTree2()
    //   TreeNode root = TreeUtils.constructBinaryTree(Arrays.asList(2, 1, 3));
    //   assertThat(new Solution().invertTree(root).toString(), equalTo("2,3,1"));
    #[test]
    fn test_invert_tree2() {
        // TODO: 翻译 Java 测试
    }
}
