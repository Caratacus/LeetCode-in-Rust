// Tests for Problem 0111: Minimum Depth of Binary Tree
// Java reference: src/test/java/g0101_0200/s0111_minimum_depth_of_binary_tree/SolutionTest.java

use leetcode_in_rust::s0111::minimum_depth_of_binary_tree::Solution;
use leetcode_in_rust::utils::tree_utils::tree_from_vec;

#[test]
fn test_min_depth() {
    let root = tree_from_vec(vec![Some(3), Some(9), Some(20), None, None, Some(15), Some(7)]);
    assert_eq!(Solution::min_depth(root), 2);
}

#[test]
fn test_min_depth2() {
    let root = tree_from_vec(vec![Some(2), None, Some(3), None, Some(4), None, Some(5), None, Some(6)]);
    assert_eq!(Solution::min_depth(root), 5);
}

#[test]
fn test_min_depth3() {
    let root = tree_from_vec(vec![]);
    assert_eq!(Solution::min_depth(root), 0);
}
