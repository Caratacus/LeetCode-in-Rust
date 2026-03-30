// Problem 1448: count good nodes in binary tree

use crate::common::tree_node::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution;

impl Solution {
    pub fn good_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void goodNodes()
    //   TreeNode root = TreeNode.create(Arrays.asList(3, 1, 4, 3, null, 1, 5));
    //   assertThat(new Solution().goodNodes(root), equalTo(4));
    #[test]
    fn test_good_nodes() {
        // TODO: 翻译 Java 测试
    }

    // Java: void goodNodes2()
    //   TreeNode root = TreeNode.create(Arrays.asList(3, 3, null, 4, 2));
    //   assertThat(new Solution().goodNodes(root), equalTo(3));
    #[test]
    fn test_good_nodes2() {
        // TODO: 翻译 Java 测试
    }

    // Java: void goodNodes3()
    //   TreeNode root = TreeNode.create(Collections.singletonList(1));
    //   assertThat(new Solution().goodNodes(root), equalTo(1));
    #[test]
    fn test_good_nodes3() {
        // TODO: 翻译 Java 测试
    }
}
