// Tests for Problem 0671: Second Minimum Node In a Binary Tree
// Java reference: src/test/java/g0601_0700/s0671_second_minimum_node_in_a_binary_tree/SolutionTest.java

use leetcode_in_rust::s0671::second_minimum_node_in_a_binary_tree::Solution;
use leetcode_in_rust::utils::tree_utils::tree_from_vec;

#[test]
fn test_find_second_minimum_value() {
    // Tree: [2, 2, 5, null, null, 5, 7]
    let root = tree_from_vec(vec![Some(2), Some(2), Some(5), None, None, Some(5), Some(7)]);
    assert_eq!(Solution::find_second_minimum_value(root), 5);
}

#[test]
fn test_find_second_minimum_value2() {
    // Tree: [2, 2, 2]
    let root = tree_from_vec(vec![Some(2), Some(2), Some(2)]);
    assert_eq!(Solution::find_second_minimum_value(root), -1);
}
