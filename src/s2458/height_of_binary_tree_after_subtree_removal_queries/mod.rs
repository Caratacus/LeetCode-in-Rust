// Problem 2458: height of binary tree after subtree removal queries

use crate::common::tree_node::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution;

impl Solution {
    pub fn tree_queries(root: Option<Rc<RefCell<TreeNode>>>, queries: Vec<i32>) -> Vec<i32> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void treeQueries()
    //   assertThat(
    //   new Solution()
    //   .treeQueries(
    //   TreeNode.create(
    //   Arrays.asList(
    //   ... (4 more lines)
    #[test]
    fn test_tree_queries() {
        // TODO: 翻译 Java 测试
    }

    // Java: void treeQueries2()
    //   assertThat(
    //   new Solution()
    //   .treeQueries(
    //   TreeNode.create(Arrays.asList(5, 8, 9, 2, 1, 3, 7, 4, 6)),
    //   new int[] {3, 2, 4, 8}),
    //   ... (1 more lines)
    #[test]
    fn test_tree_queries2() {
        // TODO: 翻译 Java 测试
    }
}
