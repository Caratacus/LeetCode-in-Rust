// Problem 0863: all nodes distance k in binary tree

use crate::common::tree_node::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution;

impl Solution {
    pub fn distance_k(
        root: Option<Rc<RefCell<TreeNode>>>,
        target: Option<Rc<RefCell<TreeNode>>>,
        k: i32,
    ) -> Vec<i32> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void distanceK()
    //   assertThat(
    //   new Solution()
    //   .distanceK(
    //   TreeUtils.constructBinaryTree(
    //   Arrays.asList(3, 5, 1, 6, 2, 0, 8, null, null, 7, 4)),
    //   ... (3 more lines)
    #[test]
    fn test_distance_k() {
        // TODO: 翻译 Java 测试
    }

    // Java: void distanceK2()
    //   assertThat(
    //   new Solution()
    //   .distanceK(
    //   TreeUtils.constructBinaryTree(Arrays.asList(1)),
    //   new TreeNode(1),
    //   ... (2 more lines)
    #[test]
    fn test_distance_k2() {
        // TODO: 翻译 Java 测试
    }

    // Java: void distanceK3()
    //   TreeNode root =
    //   TreeUtils.constructBinaryTree(Arrays.asList(3, 5, 1, 6, 2, 0, 8, null, null, 7, 4));
    //   assertThat(
    //   new Solution().distanceK(root, new TreeNode(3), 2),
    //   equalTo(Arrays.asList(6, 2, 0, 8)));
    #[test]
    fn test_distance_k3() {
        // TODO: 翻译 Java 测试
    }

    // Java: void distanceK4()
    //   TreeNode root =
    //   TreeUtils.constructBinaryTree(Arrays.asList(3, 5, 1, 6, 2, 0, 8, null, null, 7, 4));
    //   assertThat(new Solution().distanceK(root, new TreeNode(7), 1), equalTo(Arrays.asList(2)));
    #[test]
    fn test_distance_k4() {
        // TODO: 翻译 Java 测试
    }

    // Java: void distanceK5()
    //   TreeNode root = TreeUtils.constructBinaryTree(Arrays.asList(1, 2, 3));
    //   assertThat(new Solution().distanceK(root, new TreeNode(2), 0), equalTo(Arrays.asList(2)));
    #[test]
    fn test_distance_k5() {
        // TODO: 翻译 Java 测试
    }

    // Java: void distanceK6()
    //   TreeNode root = TreeUtils.constructBinaryTree(Arrays.asList(1, 2, 3));
    //   assertThat(
    //   new Solution().distanceK(root, new TreeNode(1), 5),
    //   equalTo(Collections.emptyList()));
    #[test]
    fn test_distance_k6() {
        // TODO: 翻译 Java 测试
    }

    // Java: void distanceK7()
    //   TreeNode root = TreeUtils.constructBinaryTree(Arrays.asList(1, 2, null, 3, null, 4));
    //   assertThat(new Solution().distanceK(root, new TreeNode(2), 2), equalTo(Arrays.asList(4)));
    #[test]
    fn test_distance_k7() {
        // TODO: 翻译 Java 测试
    }
}
