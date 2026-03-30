// Problem 1302: deepest leaves sum

use crate::common::tree_node::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution;

impl Solution {
    pub fn deepest_leaves_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void deepestLeavesSum()
    //   TreeNode treeNode =
    //   TreeNode.create(
    //   Arrays.asList(1, 2, 3, 4, 5, null, 6, 7, null, null, null, null, 8));
    //   assertThat(new Solution().deepestLeavesSum(treeNode), equalTo(15));
    #[test]
    fn test_deepest_leaves_sum() {
        // TODO: 翻译 Java 测试
    }

    // Java: void deepestLeavesSum2()
    //   TreeNode treeNode =
    //   TreeNode.create(
    //   Arrays.asList(6, 7, 8, 2, 7, 1, 3, 9, null, 1, 4, null, null, null, 5));
    //   assertThat(new Solution().deepestLeavesSum(treeNode), equalTo(19));
    #[test]
    fn test_deepest_leaves_sum2() {
        // TODO: 翻译 Java 测试
    }
}
