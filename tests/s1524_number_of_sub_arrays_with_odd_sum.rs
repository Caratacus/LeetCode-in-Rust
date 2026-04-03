// Tests for Problem 1524: Number of Sub-arrays With Odd Sum
// Java reference: src/test/java/g1501_1600/s1524_number_of_sub_arrays_with_odd_sum/SolutionTest.java

use leetcode_in_rust::s1524::number_of_sub_arrays_with_odd_sum::Solution;

#[test]
fn test_num_of_subarrays() {
    assert_eq!(Solution::num_of_subarrays(vec![1, 3, 5]), 4);
}

#[test]
fn test_num_of_subarrays2() {
    assert_eq!(Solution::num_of_subarrays(vec![2, 4, 6]), 0);
}

#[test]
fn test_num_of_subarrays3() {
    assert_eq!(Solution::num_of_subarrays(vec![1, 2, 3, 4, 5, 6, 7]), 16);
}
