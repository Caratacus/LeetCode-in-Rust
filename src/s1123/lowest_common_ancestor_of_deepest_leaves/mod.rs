// Problem 1123: lowest common ancestor of deepest leaves

use crate::common::tree_node::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution;

impl Solution {
    pub fn lca_deepest_leaves(
        root: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        todo!()
    }

    pub fn get_dep(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void lcaDeepestLeaves()
    //   assertThat(
    //   new Solution()
    //   .lcaDeepestLeaves(
    //   TreeNode.create(
    //   Arrays.asList(3, 5, 1, 6, 2, 0, 8, null, null, 7, 4)))
    //   ... (2 more lines)
    #[test]
    fn test_lca_deepest_leaves() {
        // TODO: 翻译 Java 测试
    }

    // Java: void lcaDeepestLeaves2()
    //   assertThat(
    //   new Solution().lcaDeepestLeaves(TreeNode.create(Arrays.asList(1))).toString(),
    //   equalTo("1"));
    #[test]
    fn test_lca_deepest_leaves2() {
        // TODO: 翻译 Java 测试
    }

    // Java: void lcaDeepestLeaves3()
    //   assertThat(
    //   new Solution()
    //   .lcaDeepestLeaves(TreeNode.create(Arrays.asList(0, 1, 3, null, 2)))
    //   .toString(),
    //   equalTo("2"));
    #[test]
    fn test_lca_deepest_leaves3() {
        // TODO: 翻译 Java 测试
    }
}
