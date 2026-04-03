// Tests for Problem 0754: Reach a Number
// Java reference: src/test/java/g0701_0800/s0754_reach_a_number/SolutionTest.java

use leetcode_in_rust::s0754::reach_a_number::Solution;

#[test]
fn test_reach_number() {
    assert_eq!(Solution::reach_number(2), 3);
}

#[test]
fn test_reach_number2() {
    assert_eq!(Solution::reach_number(3), 2);
}
