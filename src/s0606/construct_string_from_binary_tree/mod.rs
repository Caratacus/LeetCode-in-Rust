// Problem 0606: construct string from binary tree

use crate::common::tree_node::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution;

impl Solution {
    pub fn tree2str(t: Option<Rc<RefCell<TreeNode>>>) -> String {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void tree2str()
    //   TreeNode treeNode = TreeNode.create(Arrays.asList(1, 2, 3, 4));
    //   assertThat(new Solution().tree2str(treeNode), equalTo("1(2(4))(3)"));
    #[test]
    fn test_tree2str() {
        // TODO: 翻译 Java 测试
    }

    // Java: void tree2str2()
    //   TreeNode treeNode = TreeNode.create(Arrays.asList(1, 2, 3, null, 4));
    //   assertThat(new Solution().tree2str(treeNode), equalTo("1(2()(4))(3)"));
    #[test]
    fn test_tree2str2() {
        // TODO: 翻译 Java 测试
    }
}
