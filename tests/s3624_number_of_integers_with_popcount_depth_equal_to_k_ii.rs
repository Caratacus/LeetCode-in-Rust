// Tests for Problem 3624: Number of Integers with Popcount Depth Equal to K II
// Java reference: src/test/java/g3601_3700/s3624_number_of_integers_with_popcount_depth_equal_to_k_ii/SolutionTest.java
use leetcode_in_rust::s3624::number_of_integers_with_popcount_depth_equal_to_k_ii::Solution;
#[test]
fn test_popcount_depth() {
    assert_eq!(Solution::popcount_depth(vec![2i64, 4i64], vec![vec![1i64, 0, 1, 1], vec![2, 1, 1], vec![1, 0, 1, 0]]), vec![2, 1]);
}
#[test]
fn test_popcount_depth2() {
    assert_eq!(Solution::popcount_depth(vec![3i64, 5i64, 6i64], vec![vec![1i64, 0, 2, 2], vec![2, 1, 4], vec![1, 1, 2, 1], vec![1, 0, 1, 0]]), vec![3, 1, 0]);
}
#[test]
fn test_popcount_depth3() {
    assert_eq!(Solution::popcount_depth(vec![1i64, 2i64], vec![vec![1i64, 0, 1, 1], vec![2, 0, 3], vec![1, 0, 0, 1], vec![1, 0, 0, 2]]), vec![1, 0, 1]);
}
