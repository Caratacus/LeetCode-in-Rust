// Tests for Problem 1110: Delete Nodes And Return Forest
// Java reference: src/test/java/g1101_1200/s1110_delete_nodes_and_return_forest/SolutionTest.java

use leetcode_in_rust::s1110::delete_nodes_and_return_forest::Solution;
use leetcode_in_rust::utils::tree_utils::{tree_from_vec, tree_to_vec};

#[test]
fn test_del_nodes() {
    let root = tree_from_vec(vec![Some(1), Some(2), Some(3), Some(4), Some(5), Some(6), Some(7)]);
    let result = Solution::del_nodes(root, vec![3, 5]);
    let results: Vec<Vec<Option<i32>>> = result.into_iter().map(|t| tree_to_vec(t)).collect();
    assert_eq!(results.len(), 3);
}

#[test]
fn test_del_nodes2() {
    let root = tree_from_vec(vec![Some(1), Some(2), Some(4), None, Some(3)]);
    let result = Solution::del_nodes(root, vec![3]);
    let results: Vec<Vec<Option<i32>>> = result.into_iter().map(|t| tree_to_vec(t)).collect();
    assert_eq!(results.len(), 1);
}
