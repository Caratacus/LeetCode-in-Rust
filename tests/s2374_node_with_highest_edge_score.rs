// Tests for Problem 2374: Node With Highest Edge Score
// Java reference: src/test/java/g2301_2400/s2374_node_with_highest_edge_score/SolutionTest.java

use leetcode_in_rust::s2374::node_with_highest_edge_score::Solution;

#[test]
fn test_edge_score() {
    assert_eq!(Solution::edge_score(vec![1, 0, 0, 0, 0, 7, 7, 5]), 7);
}

#[test]
fn test_edge_score2() {
    assert_eq!(Solution::edge_score(vec![2, 0, 0, 2]), 0);
}
