// Tests for Problem 0098: Validate Binary Search Tree
// Java reference: src/test/java/g0001_0100/s0098_validate_binary_search_tree/SolutionTest.java

use leetcode_in_rust::s0098::validate_binary_search_tree::Solution;
use leetcode_in_rust::utils::tree_utils::tree_from_vec;

#[test]
fn test_is_valid_bst() {
    let root = tree_from_vec(vec![Some(2), Some(1), Some(3)]);
    assert_eq!(Solution::is_valid_bst(root), true);
}

#[test]
fn test_is_valid_bst2() {
    let root = tree_from_vec(vec![Some(5), Some(1), Some(4), None, None, Some(3), Some(6)]);
    assert_eq!(Solution::is_valid_bst(root), false);
}

#[test]
fn test_is_valid_bst3() {
    let root = tree_from_vec(vec![Some(2), Some(2), Some(2)]);
    assert_eq!(Solution::is_valid_bst(root), false);
}
