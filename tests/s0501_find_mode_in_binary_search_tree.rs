// Tests for Problem 0501: Find Mode in Binary Search Tree
// Java reference: src/test/java/g0501_0600/s0501_find_mode_in_binary_search_tree/SolutionTest.java

use leetcode_in_rust::s0501::find_mode_in_binary_search_tree::Solution;
use leetcode_in_rust::utils::tree_utils::tree_from_vec;

#[test]
fn test_find_mode() {
    let root = tree_from_vec(vec![Some(1), None, Some(2), Some(2)]);
    let mut result = Solution::find_mode(root);
    result.sort();
    assert_eq!(result, vec![2]);
}

#[test]
fn test_find_mode2() {
    let root = tree_from_vec(vec![Some(0)]);
    let mut result = Solution::find_mode(root);
    result.sort();
    assert_eq!(result, vec![0]);
}
