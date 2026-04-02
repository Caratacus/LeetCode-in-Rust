// Tests for Problem 0343: Integer Break
// Java reference: src/test/java/g0301_0400/s0343_integer_break/SolutionTest.java

use leetcode_in_rust::s0343::integer_break::Solution;

#[test]
fn test_integer_break() {
    assert_eq!(Solution::integer_break(2), 1);
}

#[test]
fn test_integer_break2() {
    assert_eq!(Solution::integer_break(10), 36);
}
