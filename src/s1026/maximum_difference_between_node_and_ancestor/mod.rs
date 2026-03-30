// Problem 1026: maximum difference between node and ancestor

use crate::common::tree_node::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution;

impl Solution {
    pub fn max_ancestor_diff(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void maxAncestorDiff()
    //   TreeNode treeNode =
    //   TreeNode.create(Arrays.asList(8, 3, 10, 1, 6, null, 14, null, null, 4, 7, 13));
    //   assertThat(new Solution().maxAncestorDiff(treeNode), equalTo(7));
    #[test]
    fn test_max_ancestor_diff() {
        // TODO: 翻译 Java 测试
    }

    // Java: void maxAncestorDiff2()
    //   TreeNode treeNode = TreeNode.create(Arrays.asList(1, null, 2, null, 0, 3));
    //   assertThat(new Solution().maxAncestorDiff(treeNode), equalTo(3));
    #[test]
    fn test_max_ancestor_diff2() {
        // TODO: 翻译 Java 测试
    }
}
