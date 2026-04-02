// Tests for Problem 0209: Minimum Size Subarray Sum
// Java reference: src/test/java/g0201_0300/s0209_minimum_size_subarray_sum/SolutionTest.java

use leetcode_in_rust::s0209::minimum_size_subarray_sum::Solution;

#[test]
fn test_min_sub_array_len() {
    assert_eq!(Solution::min_sub_array_len(7, vec![2, 3, 1, 2, 4, 3]), 2);
}

#[test]
fn test_min_sub_array_len2() {
    assert_eq!(Solution::min_sub_array_len(4, vec![1, 4, 4]), 1);
}

#[test]
fn test_min_sub_array_len3() {
    assert_eq!(Solution::min_sub_array_len(11, vec![1, 1, 1, 1, 1, 1, 1, 1]), 0);
}
