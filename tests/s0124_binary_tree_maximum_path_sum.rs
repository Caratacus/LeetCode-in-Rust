// Tests for Problem 0124: Binary Tree Maximum Path Sum
// Java reference: src/test/java/g0121_0200/s0124_binary_tree_maximum_path_sum/SolutionTest.java

use leetcode_in_rust::s0124::binary_tree_maximum_path_sum::Solution;
use leetcode_in_rust::utils::tree_utils::tree_from_vec;

#[test]
fn test_max_path_sum() {
    let root = tree_from_vec(vec![Some(1), Some(2), Some(3)]);
    assert_eq!(Solution::max_path_sum(root), 6);
}

#[test]
fn test_max_path_sum2() {
    let root = tree_from_vec(vec![Some(-10), Some(9), Some(20), None, None, Some(15), Some(7)]);
    assert_eq!(Solution::max_path_sum(root), 42);
}
