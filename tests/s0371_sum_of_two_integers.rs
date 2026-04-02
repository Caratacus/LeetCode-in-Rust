// Tests for Problem 0371: Sum of Two Integers
// Java reference: src/test/java/g0301_0400/s0371_sum_of_two_integers/SolutionTest.java

use leetcode_in_rust::s0371::sum_of_two_integers::Solution;

#[test]
fn test_get_sum() {
    assert_eq!(Solution::get_sum(1, 2), 3);
}

#[test]
fn test_get_sum2() {
    assert_eq!(Solution::get_sum(2, 3), 5);
}
