// Tests for Problem 1588: Sum of All Odd Length Subarrays
// Java reference: src/test/java/g1501_1600/s1588_sum_of_all_odd_length_subarrays/SolutionTest.java

use leetcode_in_rust::s1588::sum_of_all_odd_length_subarrays::Solution;

#[test]
fn test_sum_odd_length_subarrays() {
    assert_eq!(Solution::sum_odd_length_subarrays(vec![1, 4, 2, 5, 3]), 58);
}

#[test]
fn test_sum_odd_length_subarrays2() {
    assert_eq!(Solution::sum_odd_length_subarrays(vec![1, 2]), 3);
}
