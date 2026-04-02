// Tests for Problem 0523: Continuous Subarray Sum
// Java reference: src/test/java/g0501_0600/s0523_continuous_subarray_sum/SolutionTest.java

use leetcode_in_rust::s0523::continuous_subarray_sum::Solution;

#[test]
fn test_check_subarray_sum() {
    assert_eq!(Solution::check_subarray_sum(vec![23, 2, 4, 6, 7], 6), true);
}

#[test]
fn test_check_subarray_sum2() {
    assert_eq!(Solution::check_subarray_sum(vec![23, 2, 6, 4, 7], 6), true);
}

#[test]
fn test_check_subarray_sum3() {
    assert_eq!(Solution::check_subarray_sum(vec![23, 2, 6, 4, 7], 13), false);
}
