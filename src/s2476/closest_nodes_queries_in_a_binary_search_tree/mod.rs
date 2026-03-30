// Problem 2476: closest nodes queries in a binary search tree

use crate::common::tree_node::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution;

impl Solution {
    pub fn closest_nodes(root: Option<Rc<RefCell<TreeNode>>>, queries: Vec<i32>) -> Vec<Vec<i32>> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void closestNodes()
    //   assertThat(
    //   new Solution()
    //   .closestNodes(
    //   TreeNode.create(
    //   Arrays.asList(
    //   ... (6 more lines)
    #[test]
    fn test_closest_nodes() {
        // TODO: 翻译 Java 测试
    }

    // Java: void closestNodes2()
    //   assertThat(
    //   new Solution()
    //   .closestNodes(
    //   TreeNode.create(Arrays.asList(4, null, 9)),
    //   Collections.singletonList(3)),
    //   ... (1 more lines)
    #[test]
    fn test_closest_nodes2() {
        // TODO: 翻译 Java 测试
    }
}
