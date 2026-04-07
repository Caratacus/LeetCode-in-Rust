// Tests for Problem 2395: Find Subarrays With Equal Sum
// Java reference: src/test/java/g2301_2400/s2395_find_subarrays_with_equal_sum/SolutionTest.java

use leetcode_in_rust::s2395::find_subarrays_with_equal_sum::Solution;

#[test]
fn test_find_subarrays() {
    assert_eq!(Solution::find_subarrays(vec![4, 2, 4]), true);
}

#[test]
fn test_find_subarrays2() {
    assert_eq!(Solution::find_subarrays(vec![1, 2, 3, 4, 5]), false);
}

#[test]
fn test_find_subarrays3() {
    assert_eq!(Solution::find_subarrays(vec![0, 0, 0]), true);
}
