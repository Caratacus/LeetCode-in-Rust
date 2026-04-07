// Tests for Problem 2048: Next Greater Numerically Balanced Number
// Java reference: src/test/java/g2001_2100/s2048_next_greater_numerically_balanced_number/SolutionTest.java

use leetcode_in_rust::s2048::next_greater_numerically_balanced_number::Solution;

#[test]
fn test_next_beautiful_number() {
    assert_eq!(Solution::next_beautiful_number(1), 22);
}

#[test]
fn test_next_beautiful_number2() {
    assert_eq!(Solution::next_beautiful_number(1000), 1333);
}

#[test]
fn test_next_beautiful_number3() {
    assert_eq!(Solution::next_beautiful_number(3000), 3133);
}
