// Tests for Problem 3651: Minimum Cost Path with Teleportations
// Java reference: src/test/java/g3601_3700/s3651_minimum_cost_path_with_teleportations/SolutionTest.java
use leetcode_in_rust::s3651::minimum_cost_path_with_teleportations::Solution;
#[test]
fn test_min_cost() {
    assert_eq!(Solution::min_cost(vec![vec![1, 3, 3], vec![2, 5, 4], vec![4, 3, 5]], 2), 7);
}
#[test]
fn test_min_cost2() {
    assert_eq!(Solution::min_cost(vec![vec![1, 2], vec![2, 3], vec![3, 4]], 1), 9);
}
