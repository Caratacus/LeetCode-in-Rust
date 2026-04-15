// Tests for Problem 3547: Maximum Sum of Edge Values in a Graph
// Java reference: src/test/java/g3501_3600/s3547_maximum_sum_of_edge_values_in_a_graph/SolutionTest.java

use leetcode_in_rust::s3547::maximum_sum_of_edge_values_in_a_graph::Solution;

#[test]
fn test_max_score() {
    assert_eq!(Solution::max_score(7, vec![vec![0, 1], vec![1, 2], vec![2, 0], vec![3, 4], vec![4, 5], vec![5, 6]]), 130i64);
}

#[test]
fn test_max_score2() {
    assert_eq!(Solution::max_score(6, vec![vec![0, 3], vec![4, 5], vec![2, 0], vec![1, 3], vec![2, 4], vec![1, 5]]), 82i64);
}
