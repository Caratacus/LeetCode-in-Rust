// Tests for Problem 0637: Average of Levels in Binary Tree
// Java reference: src/test/java/g0601_0700/s0637_average_of_levels_in_binary_tree/SolutionTest.java

use leetcode_in_rust::s0637::average_of_levels_in_binary_tree::Solution;
use leetcode_in_rust::utils::tree_utils::tree_from_vec;

#[test]
fn test_average_of_levels() {
    // TreeNode treeNode = TreeNode.create(Arrays.asList(3, 9, 20, null, null, 15, 7));
    let tree = tree_from_vec(vec![Some(3), Some(9), Some(20), None, None, Some(15), Some(7)]);
    let result = Solution::average_of_levels(tree);
    assert_eq!(result, vec![3.0, 14.5, 11.0]);
}

#[test]
fn test_average_of_levels2() {
    // TreeNode treeNode = TreeNode.create(Arrays.asList(3, 9, 20, 15, 7));
    let tree = tree_from_vec(vec![Some(3), Some(9), Some(20), Some(15), Some(7)]);
    let result = Solution::average_of_levels(tree);
    assert_eq!(result, vec![3.0, 14.5, 11.0]);
}
