// Tests for Problem 2119: A Number After a Double Reversal
// Java reference: src/test/java/g2101_2200/s2119_a_number_after_a_double_reversal/SolutionTest.java

use leetcode_in_rust::s2119::a_number_after_a_double_reversal::Solution;

#[test]
fn test_is_same_after_reversals() {
    assert_eq!(Solution::is_same_after_reversals(526), true);
}

#[test]
fn test_is_same_after_reversals2() {
    assert_eq!(Solution::is_same_after_reversals(1800), false);
}

#[test]
fn test_is_same_after_reversals3() {
    assert_eq!(Solution::is_same_after_reversals(0), true);
}
