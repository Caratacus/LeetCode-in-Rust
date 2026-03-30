// Problem 2096: step by step directions from a binary tree node to another

use crate::common::tree_node::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution;

impl Solution {
    pub fn get_directions(
        root: Option<Rc<RefCell<TreeNode>>>,
        start_value: i32,
        dest_value: i32,
    ) -> String {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void getDirections()
    //   assertThat(
    //   new Solution()
    //   .getDirections(
    //   TreeNode.create(Arrays.asList(5, 1, 2, 3, null, 6, 4)), 3, 6),
    //   equalTo("UURL"));
    #[test]
    fn test_get_directions() {
        // TODO: 翻译 Java 测试
    }

    // Java: void getDirections2()
    //   assertThat(
    //   new Solution().getDirections(TreeNode.create(Arrays.asList(2, 1)), 2, 1),
    //   equalTo("L"));
    #[test]
    fn test_get_directions2() {
        // TODO: 翻译 Java 测试
    }
}
