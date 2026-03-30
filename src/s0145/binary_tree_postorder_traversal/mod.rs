// Problem 0145: binary tree postorder traversal

use crate::common::tree_node::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution;

impl Solution {
    pub fn postorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void postorderTraversal()
    //   TreeNode treeNode = new TreeNode(1);
    //   treeNode.right = new TreeNode(2);
    //   treeNode.right.left = new TreeNode(3);
    //   assertThat(new Solution().postorderTraversal(treeNode), equalTo(Arrays.asList(3, 2, 1)));
    #[test]
    fn test_postorder_traversal() {
        // TODO: 翻译 Java 测试
    }

    // Java: void postorderTraversal2()
    //   assertThat(new Solution().postorderTraversal(null), equalTo(Collections.emptyList()));
    #[test]
    fn test_postorder_traversal2() {
        // TODO: 翻译 Java 测试
    }

    // Java: void postorderTraversal3()
    //   assertThat(
    //   new Solution().postorderTraversal(new TreeNode(1)),
    //   equalTo(Collections.singletonList(1)));
    #[test]
    fn test_postorder_traversal3() {
        // TODO: 翻译 Java 测试
    }
}
