// Tests for Problem 3364: Minimum Positive Sum Subarray
// Java reference: src/test/java/g3301_3400/s3364_minimum_positive_sum_subarray/SolutionTest.java

use leetcode_in_rust::s3364::minimum_positive_sum_subarray::Solution;

#[test]
fn test_minimum_sum_subarray() {
    assert_eq!(Solution::minimum_sum_subarray(vec![3, -2, 1, 4], 2, 3), 1);
}

#[test]
fn test_minimum_sum_subarray2() {
    assert_eq!(Solution::minimum_sum_subarray(vec![-2, 2, -3, 1], 2, 3), -1);
}

#[test]
fn test_minimum_sum_subarray3() {
    assert_eq!(Solution::minimum_sum_subarray(vec![1, 2, 3, 4], 2, 4), 3);
}
