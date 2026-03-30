// Problem 0501: find mode in binary search tree

use crate::common::tree_node::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution;

impl Solution {
    pub fn find_mode(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void findMode()
    //   TreeNode treeNode = new TreeNode(1);
    //   treeNode.right = new TreeNode(2);
    //   treeNode.right.left = new TreeNode(2);
    //   int[] expected = new int[] {2};
    //   assertThat(new Solution().findMode(treeNode), equalTo(expected));
    #[test]
    fn test_find_mode() {
        // TODO: 翻译 Java 测试
    }

    // Java: void findMode2()
    //   TreeNode treeNode = new TreeNode(0);
    //   int[] expected = new int[] {0};
    //   assertThat(new Solution().findMode(treeNode), equalTo(expected));
    #[test]
    fn test_find_mode2() {
        // TODO: 翻译 Java 测试
    }
}
