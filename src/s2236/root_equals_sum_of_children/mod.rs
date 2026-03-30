// Problem 2236: root equals sum of children

use crate::common::tree_node::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution;

impl Solution {
    pub fn check_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void checkTree()
    //   TreeNode treeNode = TreeNode.create(Arrays.asList(10, 4, 6));
    //   assertThat(new Solution().checkTree(treeNode), equalTo(true));
    #[test]
    fn test_check_tree() {
        // TODO: 翻译 Java 测试
    }

    // Java: void checkTree2()
    //   TreeNode treeNode = TreeNode.create(Arrays.asList(5, 3, 1));
    //   assertThat(new Solution().checkTree(treeNode), equalTo(false));
    #[test]
    fn test_check_tree2() {
        // TODO: 翻译 Java 测试
    }
}
