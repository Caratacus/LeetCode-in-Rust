// Tests for Problem 1038: Binary Search Tree to Greater Sum Tree
// Java reference: src/test/java/g1001_1100/s1038_binary_search_tree_to_greater_sum_tree/SolutionTest.java

use leetcode_in_rust::s1038::binary_search_tree_to_greater_sum_tree::Solution;
use leetcode_in_rust::utils::tree_utils::{tree_from_vec, tree_to_vec};

#[test]
fn test_bst_to_gst() {
    let root = tree_from_vec(vec![
        Some(4),
        Some(1),
        Some(6),
        Some(0),
        Some(2),
        Some(5),
        Some(7),
        None,
        None,
        None,
        Some(3),
        None,
        None,
        None,
        Some(8),
    ]);
    let expected = tree_from_vec(vec![
        Some(30),
        Some(36),
        Some(21),
        Some(36),
        Some(35),
        Some(26),
        Some(15),
        None,
        None,
        None,
        Some(33),
        None,
        None,
        None,
        Some(8),
    ]);
    assert_eq!(tree_to_vec(Solution::bst_to_gst(root)), tree_to_vec(expected));
}

#[test]
fn test_bst_to_gst2() {
    let root = tree_from_vec(vec![Some(0), None, Some(1)]);
    let expected = tree_from_vec(vec![Some(1), None, Some(1)]);
    assert_eq!(tree_to_vec(Solution::bst_to_gst(root)), tree_to_vec(expected));
}
