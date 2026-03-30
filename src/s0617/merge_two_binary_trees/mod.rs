// Problem 0617: merge two binary trees

use crate::common::tree_node::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution;

impl Solution {
    pub fn merge_trees(
        root1: Option<Rc<RefCell<TreeNode>>>,
        root2: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void mergeTrees()
    //   TreeNode root1 = TreeNode.create(Arrays.asList(1, 3, 2, 5));
    //   TreeNode root2 = TreeNode.create(Arrays.asList(2, 1, 3, null, 4, null, 7));
    //   assertThat(new Solution().mergeTrees(root1, root2).toString(), equalTo("3,4,5,4,5,null,7"));
    #[test]
    fn test_merge_trees() {
        // TODO: 翻译 Java 测试
    }

    // Java: void mergeTrees2()
    //   TreeNode root1 = TreeNode.create(Arrays.asList(1));
    //   TreeNode root2 = TreeNode.create(Arrays.asList(1, 2));
    //   assertThat(new Solution().mergeTrees(root1, root2).toString(), equalTo("2,2,null"));
    #[test]
    fn test_merge_trees2() {
        // TODO: 翻译 Java 测试
    }
}
