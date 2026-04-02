// Tests for Problem 0949: Largest Time for Given Digits
// Java reference: src/test/java/g0901_1000/s0949_largest_time_for_given_digits/SolutionTest.java

use leetcode_in_rust::s0949::largest_time_for_given_digits::Solution;

#[test]
fn test_largest_time_from_digits() {
    assert_eq!(Solution::largest_time_from_digits(vec![1, 2, 3, 4]), "23:41".to_string());
}

#[test]
fn test_largest_time_from_digits2() {
    assert_eq!(Solution::largest_time_from_digits(vec![5, 5, 5, 5]), "".to_string());
}
