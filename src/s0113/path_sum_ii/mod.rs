// Problem 0113: path sum ii

use crate::common::tree_node::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution;

impl Solution {
    pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> Vec<Vec<i32>> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void pathSum()
    //   TreeNode root =
    //   TreeUtils.constructBinaryTree(
    //   Arrays.asList(5, 4, 8, 11, null, 13, 4, 7, 2, null, null, 5, 1));
    //   assertThat(
    //   new Solution().pathSum(root, 22),
    //   ... (1 more lines)
    #[test]
    fn test_path_sum() {
        // TODO: 翻译 Java 测试
    }

    // Java: void pathSum2()
    //   TreeNode root = TreeUtils.constructBinaryTree(Arrays.asList(1, 2, 3));
    //   assertThat(new Solution().pathSum(root, 5), equalTo(ArrayUtils.getLists(new int[][] {})));
    #[test]
    fn test_path_sum2() {
        // TODO: 翻译 Java 测试
    }

    // Java: void pathSum3()
    //   TreeNode root = TreeUtils.constructBinaryTree(Arrays.asList(1, 2));
    //   assertThat(new Solution().pathSum(root, 0), equalTo(ArrayUtils.getLists(new int[][] {})));
    #[test]
    fn test_path_sum3() {
        // TODO: 翻译 Java 测试
    }
}
