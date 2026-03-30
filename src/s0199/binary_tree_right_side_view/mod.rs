// Problem 0199: binary tree right side view

use crate::common::tree_node::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution;

impl Solution {
    pub fn right_side_view(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void rightSideView()
    //   TreeNode left = new TreeNode(2, null, new TreeNode(5));
    //   TreeNode right = new TreeNode(3, null, new TreeNode(4));
    //   TreeNode root = new TreeNode(1, left, right);
    //   assertThat(new Solution().rightSideView(root), equalTo(Arrays.asList(1, 3, 4)));
    #[test]
    fn test_right_side_view() {
        // TODO: 翻译 Java 测试
    }

    // Java: void rightSideView2()
    //   TreeNode root = new TreeNode(1, null, new TreeNode(3));
    //   assertThat(new Solution().rightSideView(root), equalTo(Arrays.asList(1, 3)));
    #[test]
    fn test_right_side_view2() {
        // TODO: 翻译 Java 测试
    }
}
