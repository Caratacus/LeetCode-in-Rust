// Tests for Problem 1796: Second Largest Digit in a String
// Java reference: src/test/java/g1701_1800/s1796_second_largest_digit_in_a_string/SolutionTest.java

use leetcode_in_rust::s1796::second_largest_digit_in_a_string::Solution;

#[test]
fn test_second_highest() {
    assert_eq!(Solution::second_highest("dfa12321afd".to_string()), 2);
}

#[test]
fn test_second_highest2() {
    assert_eq!(Solution::second_highest("abc1111".to_string()), -1);
}
