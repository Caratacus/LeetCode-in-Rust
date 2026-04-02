// Tests for Problem 0507: Perfect Number
// Java reference: src/test/java/g0501_0600/s0507_perfect_number/SolutionTest.java

use leetcode_in_rust::s0507::perfect_number::Solution;

#[test]
fn test_check_perfect_number() {
    assert_eq!(Solution::check_perfect_number(28), true);
}

#[test]
fn test_check_perfect_number2() {
    assert_eq!(Solution::check_perfect_number(7), false);
}
