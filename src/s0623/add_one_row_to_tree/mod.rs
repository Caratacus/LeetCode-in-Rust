// Problem 0623: add one row to tree

use crate::common::tree_node::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution;

impl Solution {
    pub fn add_one_row(
        root: Option<Rc<RefCell<TreeNode>>>,
        val: i32,
        depth: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void addOneRow()
    //   TreeNode treeNode = TreeNode.create(Arrays.asList(4, 2, 6, 3, 1, 5));
    //   TreeNode expected = TreeNode.create(Arrays.asList(4, 1, 1, 2, null, null, 6, 3, 1, 5));
    //   assertThat(
    //   new Solution().addOneRow(treeNode, 1, 2).toString(), equalTo(expected.toString()));
    #[test]
    fn test_add_one_row() {
        // TODO: 翻译 Java 测试
    }

    // Java: void addOneRow2()
    //   TreeNode treeNode = TreeNode.create(Arrays.asList(4, 2, null, 3, 1));
    //   TreeNode expected = TreeNode.create(Arrays.asList(4, 2, null, 1, 1, 3, null, null, 1));
    //   assertThat(
    //   new Solution().addOneRow(treeNode, 1, 3).toString(), equalTo(expected.toString()));
    #[test]
    fn test_add_one_row2() {
        // TODO: 翻译 Java 测试
    }
}
