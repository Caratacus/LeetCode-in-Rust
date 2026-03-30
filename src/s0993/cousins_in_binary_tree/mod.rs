// Problem 0993: cousins in binary tree

use crate::common::tree_node::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution;

impl Solution {
    pub fn is_cousins(root: Option<Rc<RefCell<TreeNode>>>, x: i32, y: i32) -> bool {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void isCousins()
    //   assertThat(
    //   new Solution()
    //   .isCousins(TreeUtils.constructBinaryTree(Arrays.asList(1, 2, 3, 4)), 4, 3),
    //   equalTo(false));
    #[test]
    fn test_is_cousins() {
        // TODO: 翻译 Java 测试
    }

    // Java: void isCousins2()
    //   assertThat(
    //   new Solution()
    //   .isCousins(
    //   TreeUtils.constructBinaryTree(
    //   Arrays.asList(1, 2, 3, null, 4, null, 5)),
    //   ... (3 more lines)
    #[test]
    fn test_is_cousins2() {
        // TODO: 翻译 Java 测试
    }

    // Java: void isCousins3()
    //   assertThat(
    //   new Solution()
    //   .isCousins(
    //   TreeUtils.constructBinaryTree(Arrays.asList(1, 2, 3, null, 4)),
    //   2,
    //   ... (2 more lines)
    #[test]
    fn test_is_cousins3() {
        // TODO: 翻译 Java 测试
    }
}
