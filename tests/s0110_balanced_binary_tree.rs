// Tests for Problem 0110: Balanced Binary Tree
// Java reference: src/test/java/g0101_0200/s0110_balanced_binary_tree/SolutionTest.java

use leetcode_in_rust::s0110::balanced_binary_tree::Solution;
use leetcode_in_rust::utils::tree_utils::tree_from_vec;

#[test]
fn test_is_balanced() {
    let root = tree_from_vec(vec![Some(3), Some(9), Some(20), None, None, Some(15), Some(7)]);
    assert_eq!(Solution::is_balanced(root), true);
}

#[test]
fn test_is_balanced2() {
    let root = tree_from_vec(vec![Some(1), Some(2), Some(2), Some(3), Some(3), None, None, Some(4), Some(4)]);
    assert_eq!(Solution::is_balanced(root), false);
}

#[test]
fn test_is_balanced3() {
    let root = tree_from_vec(vec![]);
    assert_eq!(Solution::is_balanced(root), true);
}
