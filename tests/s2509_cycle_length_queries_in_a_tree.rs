// Tests for Problem 2509: Cycle Length Queries in a Tree
// Java reference: src/test/java/g2401_2500/s2509_cycle_length_queries_in_a_tree/SolutionTest.java

use leetcode_in_rust::s2509::cycle_length_queries_in_a_tree::Solution;

#[test]
fn test_cycle_length_queries() {
    assert_eq!(
        Solution::cycle_length_queries(3, vec![vec![5, 3], vec![4, 7], vec![2, 3]]),
        vec![4, 5, 3]
    );
}

#[test]
fn test_cycle_length_queries2() {
    assert_eq!(
        Solution::cycle_length_queries(2, vec![vec![1, 2]]),
        vec![2]
    );
}
