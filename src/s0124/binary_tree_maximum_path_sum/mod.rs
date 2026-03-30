// Problem 0124: binary tree maximum path sum

use crate::common::tree_node::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution;

impl Solution {
    pub fn max_path_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void maxPathSum()
    //   TreeNode treeNode = TreeNode.create(Arrays.asList(1, 2, 3));
    //   assertThat(new Solution().maxPathSum(treeNode), equalTo(6));
    #[test]
    fn test_max_path_sum() {
        // TODO: 翻译 Java 测试
    }

    // Java: void maxPathSum2()
    //   TreeNode treeNode = TreeNode.create(Arrays.asList(-10, 9, 20, null, null, 15, 7));
    //   assertThat(new Solution().maxPathSum(treeNode), equalTo(42));
    #[test]
    fn test_max_path_sum2() {
        // TODO: 翻译 Java 测试
    }
}
