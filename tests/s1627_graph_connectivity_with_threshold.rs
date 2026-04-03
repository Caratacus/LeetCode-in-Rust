// Tests for Problem 1627: Graph Connectivity With Threshold
// Java reference: src/test/java/g1601_1700/s1627_graph_connectivity_with_threshold/SolutionTest.java

use leetcode_in_rust::s1627::graph_connectivity_with_threshold::Solution;

#[test]
fn test_are_connected() {
    assert_eq!(Solution::are_connected(6, 2, vec![vec![1, 4], vec![2, 5], vec![3, 6]]), vec![false, false, true]);
}

#[test]
fn test_are_connected2() {
    assert_eq!(Solution::are_connected(6, 0, vec![vec![4, 5], vec![3, 4], vec![3, 2], vec![2, 6], vec![1, 3]]), vec![true, true, true, true, true]);
}

#[test]
fn test_are_connected3() {
    assert_eq!(Solution::are_connected(5, 1, vec![vec![4, 5], vec![4, 5], vec![3, 2], vec![2, 3], vec![3, 4]]), vec![false, false, false, false, false]);
}
