// Tests for Problem 0400: Nth Digit
// Java reference: src/test/java/g0301_0400/s0400_nth_digit/SolutionTest.java

use leetcode_in_rust::s0400::nth_digit::Solution;

#[test]
fn test_find_nth_digit() {
    assert_eq!(Solution::find_nth_digit(3), 3);
}

#[test]
fn test_find_nth_digit2() {
    assert_eq!(Solution::find_nth_digit(11), 0);
}
