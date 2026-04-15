// Tests for Problem 3636: Threshold Majority Queries
// Java reference: src/test/java/g3601_3700/s3636_threshold_majority_queries/SolutionTest.java
use leetcode_in_rust::s3636::threshold_majority_queries::Solution;
#[test]
fn test_subarray_majority() {
    assert_eq!(Solution::subarray_majority(vec![1, 1, 2, 2, 1, 1], vec![vec![0, 5, 4], vec![0, 3, 3], vec![2, 3, 2]]), vec![1, -1, 2]);
}
#[test]
fn test_subarray_majority2() {
    assert_eq!(Solution::subarray_majority(vec![3, 2, 3, 2, 3, 2, 3], vec![vec![0, 6, 4], vec![1, 5, 2], vec![2, 4, 1], vec![3, 3, 1]]), vec![3, 2, 3, 2]);
}
