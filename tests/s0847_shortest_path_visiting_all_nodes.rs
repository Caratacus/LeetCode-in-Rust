// Tests for Problem 0847: Shortest Path Visiting All Nodes
// Java reference: src/test/java/g0801_0900/s0847_shortest_path_visiting_all_nodes/SolutionTest.java

use leetcode_in_rust::s0847::shortest_path_visiting_all_nodes::Solution;

#[test]
fn test_shortest_path_length() {
    assert_eq!(
        Solution::shortest_path_length(vec![vec![1, 2, 3], vec![0], vec![0], vec![0]]),
        4
    );
}

#[test]
fn test_shortest_path_length2() {
    assert_eq!(
        Solution::shortest_path_length(vec![vec![1], vec![0, 2, 3], vec![1, 3, 4], vec![2, 1, 2]]),
        4
    );
}
