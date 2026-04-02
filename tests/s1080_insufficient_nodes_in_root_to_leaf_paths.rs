// Tests for Problem 1080: Insufficient Nodes in Root to Leaf Paths
// Java reference: src/test/java/g1001_1100/s1080_insufficient_nodes_in_root_to_leaf_paths/SolutionTest.java

use leetcode_in_rust::s1080::insufficient_nodes_in_root_to_leaf_paths::Solution;
use leetcode_in_rust::utils::tree_utils::{tree_from_vec, tree_to_vec};

#[test]
fn test_sufficient_subset() {
    let root = tree_from_vec(vec![
        Some(1),
        Some(2),
        Some(3),
        Some(4),
        Some(-99),
        Some(-99),
        Some(7),
        Some(8),
        Some(9),
        Some(-99),
        Some(-99),
        Some(12),
        Some(13),
        Some(-99),
        Some(14),
    ]);
    let result = Solution::sufficient_subset(root, 1);
    assert_eq!(
        tree_to_vec(result),
        vec![Some(1), Some(2), Some(4), Some(8), Some(9), None, Some(3), None, Some(7), None, Some(14)]
    );
}

#[test]
fn test_sufficient_subset2() {
    let root = tree_from_vec(vec![
        Some(5),
        Some(4),
        Some(8),
        Some(11),
        None,
        Some(17),
        Some(4),
        Some(7),
        Some(1),
        None,
        None,
        Some(5),
        Some(3),
    ]);
    let result = Solution::sufficient_subset(root, 22);
    assert_eq!(
        tree_to_vec(result),
        vec![Some(5), Some(4), Some(11), Some(7), None, None, Some(8), Some(17), Some(4), Some(5), None]
    );
}

#[test]
fn test_sufficient_subset3() {
    let root = tree_from_vec(vec![Some(1), Some(2), Some(-3), Some(-5), None, Some(4), None]);
    let result = Solution::sufficient_subset(root, -1);
    assert_eq!(
        tree_to_vec(result),
        vec![Some(1), None, Some(-3), Some(4), None]
    );
}
