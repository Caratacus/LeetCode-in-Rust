// Tests for Problem 3650: Minimum Cost Path with Edge Reversals
// Java reference: src/test/java/g3601_3700/s3650_minimum_cost_path_with_edge_reversals/SolutionTest.java
use leetcode_in_rust::s3650::minimum_cost_path_with_edge_reversals::Solution;
#[test]
fn test_min_cost() {
    assert_eq!(Solution::min_cost(4, vec![vec![0, 1, 3], vec![3, 1, 1], vec![2, 3, 4], vec![0, 2, 2]]), 5);
}
#[test]
fn test_min_cost2() {
    assert_eq!(Solution::min_cost(4, vec![vec![0, 2, 1], vec![2, 1, 1], vec![1, 3, 1], vec![2, 3, 3]]), 3);
}
