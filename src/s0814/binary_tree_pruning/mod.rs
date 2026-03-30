// Problem 0814: binary tree pruning

use crate::common::tree_node::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution;

impl Solution {
    pub fn prune_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void pruneTree()
    //   TreeNode treeNode = TreeNode.create(Arrays.asList(1, null, 0, 0, 1));
    //   TreeNode expected = TreeNode.create(Arrays.asList(1, null, 0, null, 1));
    //   assertThat(new Solution().pruneTree(treeNode).toString(), equalTo(expected.toString()));
    #[test]
    fn test_prune_tree() {
        // TODO: 翻译 Java 测试
    }

    // Java: void pruneTree2()
    //   TreeNode treeNode = TreeNode.create(Arrays.asList(1, 0, 1, 0, 0, 0, 1));
    //   TreeNode expected = TreeNode.create(Arrays.asList(1, null, 1, null, 1));
    //   assertThat(new Solution().pruneTree(treeNode).toString(), equalTo(expected.toString()));
    #[test]
    fn test_prune_tree2() {
        // TODO: 翻译 Java 测试
    }

    // Java: void pruneTree3()
    //   TreeNode treeNode = TreeNode.create(Arrays.asList(1, 1, 0, 1, 1, 0, 1, 0));
    //   TreeNode expected = TreeNode.create(Arrays.asList(1, 1, 0, 1, 1, null, 1));
    //   assertThat(new Solution().pruneTree(treeNode).toString(), equalTo(expected.toString()));
    #[test]
    fn test_prune_tree3() {
        // TODO: 翻译 Java 测试
    }
}
