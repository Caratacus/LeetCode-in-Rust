// Tests for Problem 0958: Check Completeness of a Binary Tree
// Java reference: src/test/java/g0901_1000/s0958_check_completeness_of_a_binary_tree/SolutionTest.java

use leetcode_in_rust::s0958::check_completeness_of_a_binary_tree::Solution;
use leetcode_in_rust::utils::tree_utils::tree_from_vec;

#[test]
fn test_is_complete_tree() {
    let root = tree_from_vec(vec![Some(1), Some(2), Some(3), Some(4), Some(5), Some(6)]);
    assert_eq!(Solution::is_complete_tree(root), true);
}

#[test]
fn test_is_complete_tree2() {
    let root = tree_from_vec(vec![Some(1), Some(2), Some(3), Some(4), Some(5), None, Some(7)]);
    assert_eq!(Solution::is_complete_tree(root), false);
}
