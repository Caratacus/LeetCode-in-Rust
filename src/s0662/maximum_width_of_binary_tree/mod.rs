// Problem 0662: maximum width of binary tree

use crate::common::tree_node::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution;

impl Solution {
    pub fn width_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void widthOfBinaryTree()
    //   TreeNode treeNode = TreeNode.create(Arrays.asList(1, 3, 2, 5, 3, null, 9));
    //   assertThat(new Solution().widthOfBinaryTree(treeNode), equalTo(4));
    #[test]
    fn test_width_of_binary_tree() {
        // TODO: 翻译 Java 测试
    }

    // Java: void widthOfBinaryTree2()
    //   TreeNode treeNode = TreeNode.create(Arrays.asList(1, 3, null, 5, 3));
    //   assertThat(new Solution().widthOfBinaryTree(treeNode), equalTo(2));
    #[test]
    fn test_width_of_binary_tree2() {
        // TODO: 翻译 Java 测试
    }

    // Java: void widthOfBinaryTree3()
    //   TreeNode treeNode = TreeNode.create(Arrays.asList(1, 3, 2, 5));
    //   assertThat(new Solution().widthOfBinaryTree(treeNode), equalTo(2));
    #[test]
    fn test_width_of_binary_tree3() {
        // TODO: 翻译 Java 测试
    }
}
