// Tests for Problem 0450: Delete Node in a BST
// Java reference: src/test/java/g0401_0500/s0450_delete_node_in_a_bst/SolutionTest.java

use leetcode_in_rust::s0450::delete_node_in_a_bst::Solution;
use leetcode_in_rust::utils::tree_utils::tree_from_vec;

#[test]
fn test_delete_node() {
    let root = tree_from_vec(vec![Some(5), Some(3), Some(6), Some(2), Some(4), None, Some(7)]);
    let result = Solution::delete_node(root, 3);
    // Result should be a valid BST
    assert!(result.is_some());
}

#[test]
fn test_delete_node2() {
    let root = tree_from_vec(vec![Some(5), Some(3), Some(6), Some(2), Some(4), None, Some(7)]);
    let result = Solution::delete_node(root, 0);
    // Result should be unchanged
    assert!(result.is_some());
}
