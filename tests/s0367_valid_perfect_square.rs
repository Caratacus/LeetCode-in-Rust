// Tests for Problem 0367: Valid Perfect Square
// Java reference: src/test/java/g0301_0400/s0367_valid_perfect_square/SolutionTest.java

use leetcode_in_rust::s0367::valid_perfect_square::Solution;

#[test]
fn test_is_perfect_square() {
    assert_eq!(Solution::is_perfect_square(16), true);
}

#[test]
fn test_is_perfect_square2() {
    assert_eq!(Solution::is_perfect_square(14), false);
}
