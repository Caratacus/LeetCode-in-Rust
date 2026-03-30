// Problem 0515: find largest value in each tree row

use crate::common::tree_node::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution;

impl Solution {
    pub fn largest_values(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void largestValues()
    //   TreeNode treeNode = TreeUtils.constructBinaryTree(Arrays.asList(1, 3, 2, 5, 3, null, 9));
    //   assertThat(new Solution().largestValues(treeNode), equalTo(Arrays.asList(1, 3, 9)));
    #[test]
    fn test_largest_values() {
        // TODO: 翻译 Java 测试
    }

    // Java: void largestValues2()
    //   TreeNode treeNode = TreeUtils.constructBinaryTree(Arrays.asList(1, 2, 3));
    //   assertThat(new Solution().largestValues(treeNode), equalTo(Arrays.asList(1, 3)));
    #[test]
    fn test_largest_values2() {
        // TODO: 翻译 Java 测试
    }
}
