// Tests for Problem 0053: Maximum Subarray
// Java reference: src/test/java/g0001_0100/s0053_maximum_subarray/SolutionTest.java

use leetcode_in_rust::s0053::maximum_subarray::Solution;

#[test]
fn test_max_sub_array() {
    assert_eq!(Solution::max_sub_array(vec![-2, 1, -3, 4, -1, 2, 1, -5, 4]), 6);
}

#[test]
fn test_max_sub_array2() {
    assert_eq!(Solution::max_sub_array(vec![1]), 1);
}

#[test]
fn test_max_sub_array3() {
    assert_eq!(Solution::max_sub_array(vec![5, 4, -1, 7, 8]), 23);
}
