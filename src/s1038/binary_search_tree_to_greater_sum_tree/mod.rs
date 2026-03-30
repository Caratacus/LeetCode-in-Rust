// Problem 1038: binary search tree to greater sum tree

use crate::common::tree_node::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution;

impl Solution {
    pub fn bst_to_gst(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void bstToGst()
    //   TreeNode root =
    //   TreeNode.create(
    //   Arrays.asList(
    //   4, 1, 6, 0, 2, 5, 7, null, null, null, 3, null, null, null, 8));
    //   TreeNode expected =
    //   ... (5 more lines)
    #[test]
    fn test_bst_to_gst() {
        // TODO: 翻译 Java 测试
    }

    // Java: void bstToGst2()
    //   TreeNode root = TreeNode.create(Arrays.asList(0, null, 1));
    //   TreeNode expected = TreeNode.create(Arrays.asList(1, null, 1));
    //   assertThat(new Solution().bstToGst(root).toString(), equalTo(expected.toString()));
    #[test]
    fn test_bst_to_gst2() {
        // TODO: 翻译 Java 测试
    }
}
