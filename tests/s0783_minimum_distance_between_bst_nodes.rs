// Tests for Problem 0783: Minimum Distance Between BST Nodes
// Java reference: src/test/java/g0701_0800/s0783_minimum_distance_between_bst_nodes/SolutionTest.java

use leetcode_in_rust::s0783::minimum_distance_between_bst_nodes::Solution;
use leetcode_in_rust::utils::tree_utils::tree_from_vec;

#[test]
fn test_min_diff_in_bst() {
    // [4, 2, 6, 1, 3, null, null]
    let root = tree_from_vec(vec![
        Some(4),
        Some(2),
        Some(6),
        Some(1),
        Some(3),
        None,
        None,
    ]);
    assert_eq!(Solution::min_diff_in_bst(root), 1);
}

#[test]
fn test_min_diff_in_bst2() {
    // [1, 0, 48, null, null, 12, 49]
    let root = tree_from_vec(vec![
        Some(1),
        Some(0),
        Some(48),
        None,
        None,
        Some(12),
        Some(49),
    ]);
    assert_eq!(Solution::min_diff_in_bst(root), 1);
}
