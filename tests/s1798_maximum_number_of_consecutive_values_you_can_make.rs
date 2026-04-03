// Tests for Problem 1798: Maximum Number of Consecutive Values You Can Make
// Java reference: src/test/java/g1701_1800/s1798_maximum_number_of_consecutive_values_you_can_make/SolutionTest.java

use leetcode_in_rust::s1798::maximum_number_of_consecutive_values_you_can_make::Solution;

#[test]
fn test_get_maximum_consecutive() {
    assert_eq!(Solution::get_maximum_consecutive(vec![1, 3]), 2);
}

#[test]
fn test_get_maximum_consecutive2() {
    assert_eq!(Solution::get_maximum_consecutive(vec![1, 1, 1, 4]), 8);
}

#[test]
fn test_get_maximum_consecutive3() {
    assert_eq!(Solution::get_maximum_consecutive(vec![1, 4, 10, 3, 1]), 20);
}
