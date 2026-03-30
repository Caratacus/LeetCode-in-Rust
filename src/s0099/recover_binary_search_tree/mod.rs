// Problem 0099: recover binary search tree

use crate::common::tree_node::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution;

impl Solution {
    pub fn recover_tree(root: Option<Rc<RefCell<TreeNode>>>) -> () {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void recoverTree()
    //   TreeNode treeNode = new TreeNode(1);
    //   treeNode.left = new TreeNode(3);
    //   treeNode.left.right = new TreeNode(2);
    //   new Solution().recoverTree(treeNode);
    //   assertThat(treeNode.toString(), equalTo("3,1,null,2,null"));
    #[test]
    fn test_recover_tree() {
        // TODO: 翻译 Java 测试
    }

    // Java: void recoverTree2()
    //   TreeNode treeNode = new TreeNode(3);
    //   treeNode.left = new TreeNode(1);
    //   treeNode.right = new TreeNode(4);
    //   treeNode.right.left = new TreeNode(2);
    //   new Solution().recoverTree(treeNode);
    //   ... (1 more lines)
    #[test]
    fn test_recover_tree2() {
        // TODO: 翻译 Java 测试
    }
}
