// Problem 0144: binary tree preorder traversal

use crate::common::tree_node::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution;

impl Solution {
    pub fn preorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void preorderTraversal()
    //   TreeNode treeNode = new TreeNode(1);
    //   treeNode.right = new TreeNode(2);
    //   treeNode.right.left = new TreeNode(3);
    //   assertThat(new Solution().preorderTraversal(treeNode), equalTo(Arrays.asList(1, 2, 3)));
    #[test]
    fn test_preorder_traversal() {
        // TODO: 翻译 Java 测试
    }

    // Java: void preorderTraversal2()
    //   assertThat(new Solution().preorderTraversal(null), equalTo(Collections.emptyList()));
    #[test]
    fn test_preorder_traversal2() {
        // TODO: 翻译 Java 测试
    }

    // Java: void preorderTraversal3()
    //   assertThat(
    //   new Solution().preorderTraversal(new TreeNode(1)),
    //   equalTo(Collections.singletonList(1)));
    #[test]
    fn test_preorder_traversal3() {
        // TODO: 翻译 Java 测试
    }
}
