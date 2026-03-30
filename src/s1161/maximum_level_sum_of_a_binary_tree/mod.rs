// Problem 1161: maximum level sum of a binary tree

use crate::common::tree_node::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution;

impl Solution {
    pub fn max_level_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void maxLevelSum()
    //   TreeNode root = TreeNode.create(Arrays.asList(1, 7, 0, 7, -8, null, null));
    //   assertThat(new Solution().maxLevelSum(root), equalTo(2));
    #[test]
    fn test_max_level_sum() {
        // TODO: 翻译 Java 测试
    }

    // Java: void maxLevelSum2()
    //   TreeNode root =
    //   TreeNode.create(
    //   Arrays.asList(989, null, 10250, 98693, -89388, null, null, null, -32127));
    //   assertThat(new Solution().maxLevelSum(root), equalTo(2));
    #[test]
    fn test_max_level_sum2() {
        // TODO: 翻译 Java 测试
    }
}
