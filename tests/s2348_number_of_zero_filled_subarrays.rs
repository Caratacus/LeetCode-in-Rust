// Tests for Problem 2348: Number of Zero Filled Subarrays
// Java reference: src/test/java/g2301_2400/s2348_number_of_zero_filled_subarrays/SolutionTest.java

use leetcode_in_rust::s2348::number_of_zero_filled_subarrays::Solution;

#[test]
fn test_zero_filled_subarray() {
    assert_eq!(Solution::zero_filled_subarray(vec![1, 3, 0, 0, 2, 0, 0, 4]), 6);
}

#[test]
fn test_zero_filled_subarray2() {
    assert_eq!(Solution::zero_filled_subarray(vec![0, 0, 0, 2, 0, 0]), 9);
}

#[test]
fn test_zero_filled_subarray3() {
    assert_eq!(Solution::zero_filled_subarray(vec![2, 10, 2019]), 0);
}
