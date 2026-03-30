// Problem 0897: increasing order search tree

use crate::common::tree_node::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution;

impl Solution {
    pub fn increasing_bst(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void increasingBST()
    //   assertThat(
    //   new Solution()
    //   .increasingBST(
    //   TreeUtils.constructBinaryTree(
    //   Arrays.asList(
    //   ... (3 more lines)
    #[test]
    fn test_increasing_bst() {
        // TODO: 翻译 Java 测试
    }

    // Java: void increasingBST2()
    //   assertThat(
    //   new Solution()
    //   .increasingBST(TreeUtils.constructBinaryTree(Arrays.asList(5, 1, 7)))
    //   .toString(),
    //   equalTo("1,null,5,null,7"));
    #[test]
    fn test_increasing_bst2() {
        // TODO: 翻译 Java 测试
    }
}
