// Tests for Problem 2476: Closest Nodes Queries in a Binary Search Tree
// Java reference: src/test/java/g2401_2500/s2476_closest_nodes_queries_in_a_binary_search_tree/SolutionTest.java

use leetcode_in_rust::s2476::closest_nodes_queries_in_a_binary_search_tree::Solution;
use leetcode_in_rust::utils::tree_utils::tree_from_vec;

#[test]
fn test_closest_nodes() {
    let root = tree_from_vec(vec![Some(6), Some(2), Some(13), Some(1), Some(4), Some(9), Some(15), None, None, None, None, None, None, Some(14)]);
    assert_eq!(
        Solution::closest_nodes(root, vec![2, 5, 16]),
        vec![vec![2, 2], vec![4, 6], vec![15, -1]]
    );
}

#[test]
fn test_closest_nodes2() {
    let root = tree_from_vec(vec![Some(4), None, Some(9)]);
    assert_eq!(
        Solution::closest_nodes(root, vec![3]),
        vec![vec![-1, 4]]
    );
}
