// Problem 1382: balance a binary search tree

use crate::common::tree_node::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution;

impl Solution {
    pub fn balance_bst(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void balanceBST()
    //   assertThat(
    //   new Solution()
    //   .balanceBST(
    //   TreeNode.create(
    //   Arrays.asList(1, null, 2, null, 3, null, 4, null, null)))
    //   ... (2 more lines)
    #[test]
    fn test_balance_bst() {
        // TODO: 翻译 Java 测试
    }

    // Java: void balanceBST2()
    //   assertThat(
    //   new Solution().balanceBST(TreeNode.create(Arrays.asList(2, 1, 3))).toString(),
    //   equalTo("2,1,3"));
    #[test]
    fn test_balance_bst2() {
        // TODO: 翻译 Java 测试
    }
}
