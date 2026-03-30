// Problem 0094: binary tree inorder traversal

use crate::common::tree_node::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution;

impl Solution {
    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void inorderTraversal()
    //   TreeNode treeNode = new TreeNode(1);
    //   TreeNode treeNode2 = new TreeNode(2);
    //   treeNode.right = treeNode2;
    //   treeNode2.left = new TreeNode(3);
    //   assertThat(new Solution().inorderTraversal(treeNode).toString(), equalTo("[1, 3, 2]"));
    #[test]
    fn test_inorder_traversal() {
        // TODO: 翻译 Java 测试
    }

    // Java: void inorderTraversal2()
    //   assertThat(new Solution().inorderTraversal(null).toString(), equalTo("[]"));
    #[test]
    fn test_inorder_traversal2() {
        // TODO: 翻译 Java 测试
    }

    // Java: void inorderTraversal3()
    //   assertThat(new Solution().inorderTraversal(new TreeNode(1)).toString(), equalTo("[1]"));
    #[test]
    fn test_inorder_traversal3() {
        // TODO: 翻译 Java 测试
    }
}
