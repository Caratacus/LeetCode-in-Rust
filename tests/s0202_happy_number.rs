// Tests for Problem 0202: Happy Number
// Java reference: src/test/java/g0201_0300/s0202_happy_number/SolutionTest.java

use leetcode_in_rust::s0202::happy_number::Solution;

#[test]
fn test_is_happy() {
    assert_eq!(Solution::is_happy(19), true);
}

#[test]
fn test_is_happy2() {
    assert_eq!(Solution::is_happy(2), false);
}
