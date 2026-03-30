// Problem 0538: convert bst to greater tree

use crate::common::tree_node::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution;

impl Solution {
    pub fn convert_bst(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void convertBST()
    //   TreeNode treeNode =
    //   TreeUtils.constructBinaryTree(
    //   Arrays.asList(
    //   4, 1, 6, 0, 2, 5, 7, null, null, null, 3, null, null, null, 8));
    //   TreeNode expected =
    //   ... (5 more lines)
    #[test]
    fn test_convert_bst() {
        // TODO: 翻译 Java 测试
    }

    // Java: void convertBST2()
    //   TreeNode treeNode = TreeUtils.constructBinaryTree(Arrays.asList(0, null, 1));
    //   TreeNode expected = TreeUtils.constructBinaryTree(Arrays.asList(1, null, 1));
    //   assertThat(new Solution().convertBST(treeNode).toString(), equalTo(expected.toString()));
    #[test]
    fn test_convert_bst2() {
        // TODO: 翻译 Java 测试
    }
}
