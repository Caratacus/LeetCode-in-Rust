// Tests for Problem 3102: Minimize Manhattan Distances
// Java reference: src/test/java/g3101_3200/s3102_minimize_manhattan_distances/SolutionTest.java

use leetcode_in_rust::s3102::minimize_manhattan_distances::Solution;

#[test]
fn test_minimum_distance() {
    assert_eq!(Solution::minimum_distance(vec![vec![3, 10], vec![5, 15], vec![10, 2], vec![4, 1]]), 12);
}

#[test]
fn test_minimum_distance2() {
    assert_eq!(Solution::minimum_distance(vec![vec![1, 1], vec![1, 1], vec![1, 1]]), 0);
}
