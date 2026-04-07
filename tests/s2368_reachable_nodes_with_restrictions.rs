// Tests for Problem 2368: Reachable Nodes With Restrictions
// Java reference: src/test/java/g2301_2400/s2368_reachable_nodes_with_restrictions/SolutionTest.java

use leetcode_in_rust::s2368::reachable_nodes_with_restrictions::Solution;

#[test]
fn test_reachable_nodes() {
    let edges = vec![vec![0, 1], vec![1, 2], vec![3, 1], vec![4, 0], vec![0, 5], vec![5, 6]];
    let restricted = vec![4, 5];
    assert_eq!(Solution::reachable_nodes(7, edges, restricted), 4);
}

#[test]
fn test_reachable_nodes2() {
    let edges = vec![vec![0, 1], vec![0, 2], vec![0, 5], vec![0, 4], vec![3, 2], vec![6, 5]];
    let restricted = vec![4, 2, 1];
    assert_eq!(Solution::reachable_nodes(7, edges, restricted), 3);
}
