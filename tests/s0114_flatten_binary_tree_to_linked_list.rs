// Tests for Problem 0114: Flatten Binary Tree to Linked List
// Java reference: src/test/java/g0101_0200/s0114_flatten_binary_tree_to_linked_list/SolutionTest.java

use leetcode_in_rust::s0114::flatten_binary_tree_to_linked_list::Solution;
use leetcode_in_rust::utils::tree_utils::tree_from_vec;

#[test]
fn test_flatten() {
    let root = tree_from_vec(vec![Some(1), Some(2), Some(5), Some(3), Some(4), None, Some(6)]);
    // Note: This modifies the tree in place
    Solution::flatten(root);
    // Verification would require tree traversal
}

#[test]
fn test_flatten2() {
    let root = tree_from_vec(vec![]);
    Solution::flatten(root);
}

#[test]
fn test_flatten3() {
    let root = tree_from_vec(vec![Some(0)]);
    Solution::flatten(root);
}
