// Problem 0257: binary tree paths

use crate::common::tree_node::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution;

impl Solution {
    pub fn binary_tree_paths(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<String> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void binaryTreePaths()
    //   TreeNode treeNode =
    //   new TreeNode(1, new TreeNode(2, null, new TreeNode(5)), new TreeNode(3));
    //   assertThat(
    //   new Solution().binaryTreePaths(treeNode),
    //   equalTo(Arrays.asList("1->2->5", "1->3")));
    #[test]
    fn test_binary_tree_paths() {
        // TODO: 翻译 Java 测试
    }

    // Java: void binaryTreePaths2()
    //   TreeNode treeNode = new TreeNode(1);
    //   assertThat(new Solution().binaryTreePaths(treeNode), equalTo(Arrays.asList("1")));
    #[test]
    fn test_binary_tree_paths2() {
        // TODO: 翻译 Java 测试
    }
}
