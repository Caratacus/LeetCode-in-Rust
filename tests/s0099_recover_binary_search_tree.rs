// Tests for Problem 0099: Recover Binary Search Tree
// Java reference: src/test/java/g0001_0100/s0099_recover_binary_search_tree/SolutionTest.java

use leetcode_in_rust::s0099::recover_binary_search_tree::Solution;
use leetcode_in_rust::utils::tree_utils::{tree_from_vec, tree_to_vec};

#[test]
fn test_recover_tree() {
    let root = tree_from_vec(vec![Some(1), Some(3), None, None, Some(2)]);
    Solution::recover_tree(root.clone());
    let result = tree_to_vec(root);
    assert_eq!(result, vec![Some(3), Some(1), None, None, Some(2)]);
}

#[test]
fn test_recover_tree2() {
    let root = tree_from_vec(vec![Some(3), Some(1), Some(4), None, None, Some(2)]);
    Solution::recover_tree(root.clone());
    let result = tree_to_vec(root);
    assert_eq!(result, vec![Some(2), Some(1), Some(4), None, None, Some(3)]);
}
