// Tests for Problem 0104: Maximum Depth of Binary Tree
// Java reference: src/test/java/g0101_0200/s0104_maximum_depth_of_binary_tree/SolutionTest.java

use leetcode_in_rust::s0104::maximum_depth_of_binary_tree::Solution;
use leetcode_in_rust::utils::tree_utils::tree_from_vec;

#[test]
fn test_max_depth() {
    let root = tree_from_vec(vec![Some(3), Some(9), Some(20), None, None, Some(15), Some(7)]);
    assert_eq!(Solution::max_depth(root), 3);
}

#[test]
fn test_max_depth2() {
    let root = tree_from_vec(vec![Some(1)]);
    assert_eq!(Solution::max_depth(root), 1);
}

#[test]
fn test_max_depth3() {
    let root = tree_from_vec(vec![]);
    assert_eq!(Solution::max_depth(root), 0);
}
