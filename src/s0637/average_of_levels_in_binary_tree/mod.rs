// Problem 0637: average of levels in binary tree

use crate::common::tree_node::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution;

impl Solution {
    pub fn average_of_levels(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<f64> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void averageOfLevels()
    //   TreeNode treeNode = TreeNode.create(Arrays.asList(3, 9, 20, null, null, 15, 7));
    //   assertThat(
    //   new Solution().averageOfLevels(treeNode),
    //   equalTo(Arrays.asList(3.00000, 14.50000, 11.00000)));
    #[test]
    fn test_average_of_levels() {
        // TODO: 翻译 Java 测试
    }

    // Java: void averageOfLevels2()
    //   TreeNode treeNode = TreeNode.create(Arrays.asList(3, 9, 20, 15, 7));
    //   assertThat(
    //   new Solution().averageOfLevels(treeNode),
    //   equalTo(Arrays.asList(3.00000, 14.50000, 11.00000)));
    #[test]
    fn test_average_of_levels2() {
        // TODO: 翻译 Java 测试
    }
}
