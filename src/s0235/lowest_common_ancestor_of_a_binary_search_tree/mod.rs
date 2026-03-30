// Problem 0235: lowest common ancestor of a binary search tree

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
    //   TreeNode leftNodeLeftNode = new TreeNode(0);
    //   TreeNode leftNodeRightNode = new TreeNode(4, new TreeNode(3), new TreeNode(5));
    //   TreeNode leftNode = new TreeNode(2, leftNodeLeftNode, leftNodeRightNode);
    //   TreeNode rightNode = new TreeNode(6, new TreeNode(7), new TreeNode(9));
    //   TreeNode root = new TreeNode(6, leftNode, rightNode);
    //   ... (3 more lines)
    #[test]
    fn test_lowest_common_ancestor() {
        // TODO: 翻译 Java 测试
    }

    // Java: void lowestCommonAncestor2()
    //   TreeNode leftNodeLeftNode = new TreeNode(0);
    //   TreeNode leftNodeRightNode = new TreeNode(4, new TreeNode(3), new TreeNode(5));
    //   TreeNode leftNode = new TreeNode(2, leftNodeLeftNode, leftNodeRightNode);
    //   TreeNode rightNode = new TreeNode(6, new TreeNode(7), new TreeNode(9));
    //   TreeNode root = new TreeNode(6, leftNode, rightNode);
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

    // Java: void lowestCommonAncestor4()
    //   TreeNode root = new TreeNode(1);
    //   assertThat(
    //   new Solution().lowestCommonAncestor(root, new TreeNode(1), new TreeNode(1)).val,
    //   equalTo(1));
    #[test]
    fn test_lowest_common_ancestor4() {
        // TODO: 翻译 Java 测试
    }

    // Java: void lowestCommonAncestor5()
    //   TreeNode root = new TreeNode(3, new TreeNode(1), new TreeNode(4));
    //   assertThat(
    //   new Solution().lowestCommonAncestor(root, new TreeNode(1), new TreeNode(1)).val,
    //   equalTo(1));
    #[test]
    fn test_lowest_common_ancestor5() {
        // TODO: 翻译 Java 测试
    }

    // Java: void lowestCommonAncestor6()
    //   TreeNode root = new TreeNode(3, new TreeNode(1), new TreeNode(4));
    //   assertThat(
    //   new Solution().lowestCommonAncestor(root, new TreeNode(4), new TreeNode(4)).val,
    //   equalTo(4));
    #[test]
    fn test_lowest_common_ancestor6() {
        // TODO: 翻译 Java 测试
    }

    // Java: void lowestCommonAncestor7()
    //   TreeNode root = new TreeNode(5, new TreeNode(3), new TreeNode(8));
    //   assertThat(
    //   new Solution().lowestCommonAncestor(root, new TreeNode(3), new TreeNode(8)).val,
    //   equalTo(5));
    #[test]
    fn test_lowest_common_ancestor7() {
        // TODO: 翻译 Java 测试
    }
}
