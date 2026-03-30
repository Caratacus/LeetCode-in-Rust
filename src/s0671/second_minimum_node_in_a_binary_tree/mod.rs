// Problem 0671: second minimum node in a binary tree

use crate::common::tree_node::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution;

impl Solution {
    pub fn find_second_minimum_value(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void findSecondMinimumValue()
    //   TreeNode treeNode = TreeNode.create(Arrays.asList(2, 2, 5, null, null, 5, 7));
    //   assertThat(new Solution().findSecondMinimumValue(treeNode), equalTo(5));
    #[test]
    fn test_find_second_minimum_value() {
        // TODO: 翻译 Java 测试
    }

    // Java: void findSecondMinimumValue2()
    //   TreeNode treeNode = TreeNode.create(Arrays.asList(2, 2, 2));
    //   assertThat(new Solution().findSecondMinimumValue(treeNode), equalTo(-1));
    #[test]
    fn test_find_second_minimum_value2() {
        // TODO: 翻译 Java 测试
    }
}
