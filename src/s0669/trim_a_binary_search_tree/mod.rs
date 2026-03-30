// Problem 0669: trim a binary search tree

use crate::common::tree_node::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution;

impl Solution {
    pub fn trim_bst(
        root: Option<Rc<RefCell<TreeNode>>>,
        l: i32,
        r: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void trimBST()
    //   TreeNode treeNode = TreeNode.create(Arrays.asList(1, 0, 2));
    //   TreeNode expected = TreeNode.create(Arrays.asList(1, null, 2));
    //   assertThat(new Solution().trimBST(treeNode, 1, 2).toString(), equalTo(expected.toString()));
    #[test]
    fn test_trim_bst() {
        // TODO: 翻译 Java 测试
    }

    // Java: void trimBST2()
    //   TreeNode treeNode = TreeNode.create(Arrays.asList(3, 0, 4, null, 2, null, null, 1));
    //   TreeNode expected = TreeNode.create(Arrays.asList(3, 2, null, 1));
    //   assertThat(new Solution().trimBST(treeNode, 1, 3).toString(), equalTo(expected.toString()));
    #[test]
    fn test_trim_bst2() {
        // TODO: 翻译 Java 测试
    }
}
