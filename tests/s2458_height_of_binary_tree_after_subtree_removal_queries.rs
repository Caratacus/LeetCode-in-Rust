// Tests for Problem 2458: Height of Binary Tree After Subtree Removal Queries
// Java reference: src/test/java/g2401_2500/s2458_height_of_binary_tree_after_subtree_removal_queries/SolutionTest.java

use leetcode_in_rust::s2458::height_of_binary_tree_after_subtree_removal_queries::Solution;
use leetcode_in_rust::utils::tree_utils::tree_from_vec;

#[test]
fn test_tree_queries() {
    let root = tree_from_vec(vec![
        Some(1), Some(3), Some(4), Some(2), Some(6), Some(5), Some(7),
        None, None, None, None, None, None, None, Some(8),
    ]);
    assert_eq!(Solution::tree_queries(root, vec![4]), vec![2]);
}

#[test]
fn test_tree_queries2() {
    let root = tree_from_vec(vec![
        Some(5), Some(8), Some(9), Some(2), Some(1), Some(3), Some(7), Some(4), Some(6),
    ]);
    assert_eq!(Solution::tree_queries(root, vec![3, 2, 4, 8]), vec![3, 2, 3, 2]);
}
