// Tests for Problem 1026: Maximum Difference Between Node and Ancestor
// Java reference: src/test/java/g1001_1100/s1026_maximum_difference_between_node_and_ancestor/SolutionTest.java

use leetcode_in_rust::s1026::maximum_difference_between_node_and_ancestor::Solution;
use leetcode_in_rust::utils::tree_utils::tree_from_vec;

#[test]
fn test_max_ancestor_diff() {
    let root = tree_from_vec(vec![
        Some(8),
        Some(3),
        Some(10),
        Some(1),
        Some(6),
        None,
        Some(14),
        None,
        None,
        Some(4),
        Some(7),
        Some(13),
    ]);
    assert_eq!(Solution::max_ancestor_diff(root), 7);
}

#[test]
fn test_max_ancestor_diff2() {
    let root = tree_from_vec(vec![Some(1), None, Some(2), None, Some(0), Some(3)]);
    assert_eq!(Solution::max_ancestor_diff(root), 3);
}
