// Problem 2471: minimum number of operations to sort a binary tree by level

use crate::common::tree_node::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution;

impl Solution {
    pub fn minimum_operations(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void minimumOperations()
    //   assertThat(
    //   new Solution()
    //   .minimumOperations(
    //   TreeNode.create(
    //   Arrays.asList(
    //   ... (3 more lines)
    #[test]
    fn test_minimum_operations() {
        // TODO: 翻译 Java 测试
    }

    // Java: void minimumOperations2()
    //   assertThat(
    //   new Solution()
    //   .minimumOperations(TreeNode.create(Arrays.asList(1, 3, 2, 7, 6, 5, 4))),
    //   equalTo(3));
    #[test]
    fn test_minimum_operations2() {
        // TODO: 翻译 Java 测试
    }

    // Java: void minimumOperations3()
    //   assertThat(
    //   new Solution().minimumOperations(TreeNode.create(Arrays.asList(1, 2, 3, 4, 5, 6))),
    //   equalTo(0));
    #[test]
    fn test_minimum_operations3() {
        // TODO: 翻译 Java 测试
    }
}
