// Tests for Problem 2316: Count Unreachable Pairs of Nodes in an Undirected Graph
// Java reference: src/test/java/g2301_2400/s2316_count_unreachable_pairs_of_nodes_in_an_undirected_graph/SolutionTest.java

use leetcode_in_rust::s2316::count_unreachable_pairs_of_nodes_in_an_undirected_graph::Solution;

#[test]
fn test_count_pairs() {
    assert_eq!(
        Solution::count_pairs(3, vec![vec![0, 1], vec![0, 2], vec![1, 2]]),
        0
    );
}

#[test]
fn test_count_pairs2() {
    assert_eq!(
        Solution::count_pairs(7, vec![vec![0, 2], vec![0, 5], vec![2, 4], vec![1, 6], vec![5, 4]]),
        14
    );
}
