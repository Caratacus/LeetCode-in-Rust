// Problem 1373: maximum sum bst in binary tree

use crate::common::tree_node::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution;

impl Solution {
    pub fn max_sum_bst(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void maxSumBST()
    //   TreeNode treeNode =
    //   TreeNode.create(
    //   Arrays.asList(
    //   1, 4, 3, 2, 4, 2, 5, null, null, null, null, null, null, 4, 6));
    //   assertThat(new Solution().maxSumBST(treeNode), equalTo(20));
    #[test]
    fn test_max_sum_bst() {
        // TODO: 翻译 Java 测试
    }

    // Java: void maxSumBST2()
    //   TreeNode treeNode = TreeNode.create(Arrays.asList(4, 3, null, 1, 2));
    //   assertThat(new Solution().maxSumBST(treeNode), equalTo(2));
    #[test]
    fn test_max_sum_bst2() {
        // TODO: 翻译 Java 测试
    }

    // Java: void maxSumBST3()
    //   TreeNode treeNode = TreeNode.create(Arrays.asList(-4, -2, -5));
    //   assertThat(new Solution().maxSumBST(treeNode), equalTo(0));
    #[test]
    fn test_max_sum_bst3() {
        // TODO: 翻译 Java 测试
    }
}
