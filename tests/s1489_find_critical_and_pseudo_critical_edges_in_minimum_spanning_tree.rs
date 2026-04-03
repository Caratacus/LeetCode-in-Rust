// Tests for Problem 1489: Find Critical and Pseudo Critical Edges in Minimum Spanning Tree
// Java reference: src/test/java/g1401_1500/s1489_find_critical_and_pseudo_critical_edges_in_minimum_spanning_tree/SolutionTest.java

use leetcode_in_rust::s1489::find_critical_and_pseudo_critical_edges_in_minimum_spanning_tree::Solution;

#[test]
fn test_find_critical_and_pseudo_critical_edges() {
    let edges = vec![vec![0, 1, 1], vec![1, 2, 1], vec![2, 3, 2], vec![0, 3, 2], vec![0, 4, 3], vec![3, 4, 3], vec![1, 4, 6]];
    let result = Solution::find_critical_and_pseudo_critical_edges(5, edges);
    assert_eq!(result, vec![vec![0, 1], vec![2, 3, 4, 5]]);
}

#[test]
fn test_find_critical_and_pseudo_critical_edges2() {
    let edges = vec![vec![0, 1, 1], vec![1, 2, 1], vec![2, 3, 1], vec![0, 3, 1]];
    let result = Solution::find_critical_and_pseudo_critical_edges(4, edges);
    assert_eq!(result, vec![vec![], vec![0, 1, 2, 3]]);
}
