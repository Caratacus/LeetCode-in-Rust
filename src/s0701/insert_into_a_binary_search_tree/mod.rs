// Problem 0701: insert into a binary search tree

use crate::common::tree_node::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution;

impl Solution {
    pub fn insert_into_bst(
        root: Option<Rc<RefCell<TreeNode>>>,
        val: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void insertIntoBST()
    //   TreeNode treeNode = TreeNode.create(Arrays.asList(4, 2, 7, 1, 3));
    //   TreeNode expected = TreeNode.create(Arrays.asList(4, 2, 7, 1, 3, 5));
    //   assertThat(
    //   new Solution().insertIntoBST(treeNode, 5).toString(), equalTo(expected.toString()));
    #[test]
    fn test_insert_into_bst() {
        // TODO: 翻译 Java 测试
    }

    // Java: void insertIntoBST2()
    //   TreeNode treeNode = TreeNode.create(Arrays.asList(40, 20, 60, 10, 30, 50, 70));
    //   TreeNode expected =
    //   TreeNode.create(Arrays.asList(40, 20, 60, 10, 30, 50, 70, null, null, 25));
    //   assertThat(
    //   new Solution().insertIntoBST(treeNode, 25).toString(),
    //   ... (1 more lines)
    #[test]
    fn test_insert_into_bst2() {
        // TODO: 翻译 Java 测试
    }

    // Java: void insertIntoBST3()
    //   TreeNode treeNode =
    //   TreeNode.create(Arrays.asList(4, 2, 7, 1, 3, null, null, null, null, null, null));
    //   TreeNode expected = TreeNode.create(Arrays.asList(4, 2, 7, 1, 3, 5));
    //   assertThat(
    //   new Solution().insertIntoBST(treeNode, 5).toString(), equalTo(expected.toString()));
    #[test]
    fn test_insert_into_bst3() {
        // TODO: 翻译 Java 测试
    }
}
