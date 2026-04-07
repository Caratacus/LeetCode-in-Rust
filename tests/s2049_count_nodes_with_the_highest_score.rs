// Tests for Problem 2049: Count Nodes With The Highest Score
// Java reference: src/test/java/g2001_2100/s2049_count_nodes_with_the_highest_score/SolutionTest.java

use leetcode_in_rust::s2049::count_nodes_with_the_highest_score::Solution;

#[test]
fn test_count_highest_score_nodes() {
    assert_eq!(Solution::count_highest_score_nodes(vec![-1, 2, 0, 2, 0]), 3);
}

#[test]
fn test_count_highest_score_nodes2() {
    assert_eq!(Solution::count_highest_score_nodes(vec![-1, 2, 0]), 2);
}
