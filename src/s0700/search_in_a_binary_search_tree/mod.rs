// Problem 0700: search in a binary search tree

use crate::common::tree_node::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution;

impl Solution {
    pub fn search_bst(
        root: Option<Rc<RefCell<TreeNode>>>,
        val: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void searchBST()
    //   TreeNode root = TreeUtils.constructBinaryTree(Arrays.asList(4, 2, 7, 1, 3));
    //   TreeNode expected = TreeUtils.constructBinaryTree(Arrays.asList(2, 1, 3));
    //   assertThat(new Solution().searchBST(root, 2).toString(), equalTo(expected.toString()));
    #[test]
    fn test_search_bst() {
        // TODO: 翻译 Java 测试
    }

    // Java: void searchBST2()
    //   TreeNode root = TreeUtils.constructBinaryTree(Arrays.asList(4, 2, 7, 1, 3));
    //   assertThat(new Solution().searchBST(root, 5), equalTo(null));
    #[test]
    fn test_search_bst2() {
        // TODO: 翻译 Java 测试
    }
}
