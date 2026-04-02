// Tests for Problem 1123: Lowest Common Ancestor of Deepest Leaves
// Java reference: src/test/java/g1101_1200/s1123_lowest_common_ancestor_of_deepest_leaves/SolutionTest.java

use leetcode_in_rust::s1123::lowest_common_ancestor_of_deepest_leaves::Solution;
use leetcode_in_rust::utils::tree_utils::{tree_from_vec, tree_to_vec};

#[test]
fn test_lca_deepest_leaves() {
    let root = tree_from_vec(vec![
        Some(3),
        Some(5),
        Some(1),
        Some(6),
        Some(2),
        Some(0),
        Some(8),
        None,
        None,
        Some(7),
        Some(4),
    ]);
    let result = Solution::lca_deepest_leaves(root);
    assert_eq!(tree_to_vec(result), vec![Some(2), Some(7), Some(4)]);
}

#[test]
fn test_lca_deepest_leaves2() {
    let root = tree_from_vec(vec![Some(1)]);
    let result = Solution::lca_deepest_leaves(root);
    assert_eq!(tree_to_vec(result), vec![Some(1)]);
}

#[test]
fn test_lca_deepest_leaves3() {
    let root = tree_from_vec(vec![Some(0), Some(1), Some(3), None, Some(2)]);
    let result = Solution::lca_deepest_leaves(root);
    assert_eq!(tree_to_vec(result), vec![Some(2)]);
}
