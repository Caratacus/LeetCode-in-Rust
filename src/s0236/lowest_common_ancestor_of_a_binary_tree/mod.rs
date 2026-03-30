// Problem 0236: lowest common ancestor of a binary tree

use crate::common::tree_node::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution;

impl Solution {
    pub fn lowest_common_ancestor(
        root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void lowestCommonAncestor()
    //   TreeNode leftNodeLeftNode = new TreeNode(6);
    //   TreeNode leftNodeRightNode = new TreeNode(2, new TreeNode(7), new TreeNode(4));
    //   TreeNode leftNode = new TreeNode(5, leftNodeLeftNode, leftNodeRightNode);
    //   TreeNode rightNode = new TreeNode(1, new TreeNode(0), new TreeNode(8));
    //   TreeNode root = new TreeNode(3, leftNode, rightNode);
    //   ... (3 more lines)
    #[test]
    fn test_lowest_common_ancestor() {
        // TODO: 翻译 Java 测试
    }

    // Java: void lowestCommonAncestor2()
    //   TreeNode leftNodeLeftNode = new TreeNode(6);
    //   TreeNode leftNodeRightNode = new TreeNode(2, new TreeNode(7), new TreeNode(4));
    //   TreeNode leftNode = new TreeNode(5, leftNodeLeftNode, leftNodeRightNode);
    //   TreeNode rightNode = new TreeNode(1, new TreeNode(0), new TreeNode(8));
    //   TreeNode root = new TreeNode(3, leftNode, rightNode);
    //   ... (3 more lines)
    #[test]
    fn test_lowest_common_ancestor2() {
        // TODO: 翻译 Java 测试
    }

    // Java: void lowestCommonAncestor3()
    //   assertThat(
    //   new Solution()
    //   .lowestCommonAncestor(
    //   new TreeNode(2, new TreeNode(1), null),
    //   new TreeNode(2),
    //   ... (3 more lines)
    #[test]
    fn test_lowest_common_ancestor3() {
        // TODO: 翻译 Java 测试
    }
}
