// Problem 0337: house robber iii

use crate::common::tree_node::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution;

impl Solution {
    pub fn rob(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void rob()
    //   TreeNode root =
    //   new TreeNode(
    //   3,
    //   new TreeNode(2, null, new TreeNode(3)),
    //   new TreeNode(3, null, new TreeNode(1)));
    //   ... (1 more lines)
    #[test]
    fn test_rob() {
        // TODO: 翻译 Java 测试
    }

    // Java: void rob2()
    //   TreeNode root =
    //   new TreeNode(
    //   3,
    //   new TreeNode(4, new TreeNode(1), new TreeNode(3)),
    //   new TreeNode(5, null, new TreeNode(1)));
    //   ... (1 more lines)
    #[test]
    fn test_rob2() {
        // TODO: 翻译 Java 测试
    }
}
