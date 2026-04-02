// Tests for Problem 0397: Integer Replacement
// Java reference: src/test/java/g0301_0400/s0397_integer_replacement/SolutionTest.java

use leetcode_in_rust::s0397::integer_replacement::Solution;

#[test]
fn test_integer_replacement() {
    assert_eq!(Solution::integer_replacement(8), 3);
}

#[test]
fn test_integer_replacement2() {
    assert_eq!(Solution::integer_replacement(7), 4);
}

#[test]
fn test_integer_replacement3() {
    assert_eq!(Solution::integer_replacement(4), 2);
}
