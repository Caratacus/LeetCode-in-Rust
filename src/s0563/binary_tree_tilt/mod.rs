// Problem 0563: binary tree tilt

use crate::common::tree_node::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution;

impl Solution {
    pub fn find_tilt(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void findTilt()
    //   TreeNode treeNode = TreeUtils.constructBinaryTree(Arrays.asList(1, 2, 3));
    //   assertThat(new Solution().findTilt(treeNode), equalTo(1));
    #[test]
    fn test_find_tilt() {
        // TODO: 翻译 Java 测试
    }

    // Java: void findTilt2()
    //   TreeNode treeNode = TreeUtils.constructBinaryTree(Arrays.asList(4, 2, 9, 3, 5, null, 7));
    //   assertThat(new Solution().findTilt(treeNode), equalTo(15));
    #[test]
    fn test_find_tilt2() {
        // TODO: 翻译 Java 测试
    }

    // Java: void findTilt3()
    //   TreeNode treeNode =
    //   TreeUtils.constructBinaryTree(Arrays.asList(21, 7, 14, 1, 1, 2, 2, 3, 3));
    //   assertThat(new Solution().findTilt(treeNode), equalTo(9));
    #[test]
    fn test_find_tilt3() {
        // TODO: 翻译 Java 测试
    }
}
