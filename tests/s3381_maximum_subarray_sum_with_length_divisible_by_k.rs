// Tests for Problem 3381: Maximum Subarray Sum With Length Divisible by K
// Java reference: src/test/java/g3301_3400/s3381_maximum_subarray_sum_with_length_divisible_by_k/SolutionTest.java

use leetcode_in_rust::s3381::maximum_subarray_sum_with_length_divisible_by_k::Solution;

#[test]
fn test_max_subarray_sum() {
    assert_eq!(Solution::max_subarray_sum(vec![1, 2], 1), 3);
}

#[test]
fn test_max_subarray_sum2() {
    assert_eq!(Solution::max_subarray_sum(vec![-1, -2, -3, -4, -5], 4), -10);
}

#[test]
fn test_max_subarray_sum3() {
    assert_eq!(Solution::max_subarray_sum(vec![-5, 1, 2, -3, 4], 2), 4);
}
