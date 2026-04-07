// Tests for Problem 2871: Split Array Into Maximum Number of Subarrays
// Java reference: src/test/java/g2801_2900/s2871_split_array_into_maximum_number_of_subarrays/SolutionTest.java

use leetcode_in_rust::s2871::split_array_into_maximum_number_of_subarrays::Solution;

#[test]
fn test_max_subarrays() {
    assert_eq!(Solution::max_subarrays(vec![1, 0, 2, 0, 1, 2]), 3);
}

#[test]
fn test_max_subarrays2() {
    assert_eq!(Solution::max_subarrays(vec![5, 7, 1, 3]), 1);
}

#[test]
fn test_max_subarrays3() {
    assert_eq!(Solution::max_subarrays(vec![5]), 1);
}
