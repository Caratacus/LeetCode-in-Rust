// Tests for Problem 3026: Maximum Good Subarray Sum
// Java reference: src/test/java/g3001_3100/s3026_maximum_good_subarray_sum/SolutionTest.java
use leetcode_in_rust::s3026::maximum_good_subarray_sum::Solution;
#[test]
fn test_maximum_subarray_sum() {
    assert_eq!(Solution::maximum_subarray_sum(vec![1, 2, 3, 4, 5, 6], 1), 11);
}
#[test]
fn test_maximum_subarray_sum2() {
    assert_eq!(Solution::maximum_subarray_sum(vec![-1, 3, 2, 4, 5], 3), 11);
}
#[test]
fn test_maximum_subarray_sum3() {
    assert_eq!(Solution::maximum_subarray_sum(vec![-1, -2, -3, -4], 2), -6);
}
