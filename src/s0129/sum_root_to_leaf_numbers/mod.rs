// Problem 0129: sum root to leaf numbers

use crate::common::tree_node::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution;

impl Solution {
    pub fn sum_numbers(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void sumNumbers()
    //   TreeNode treeNode = TreeNode.create(Arrays.asList(1, 2, 3));
    //   assertThat(new Solution().sumNumbers(treeNode), equalTo(25));
    #[test]
    fn test_sum_numbers() {
        // TODO: 翻译 Java 测试
    }

    // Java: void sumNumbers2()
    //   TreeNode treeNode = TreeNode.create(Arrays.asList(4, 9, 0, 5, 1));
    //   assertThat(new Solution().sumNumbers(treeNode), equalTo(1026));
    #[test]
    fn test_sum_numbers2() {
        // TODO: 翻译 Java 测试
    }
}
