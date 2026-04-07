// Tests for Problem 2938: Separate Black and White Balls
// Java reference: src/test/java/g2901_3000/s2938_separate_black_and_white_balls/SolutionTest.java

use leetcode_in_rust::s2938::separate_black_and_white_balls::Solution;

#[test]
fn test_minimum_steps() {
    assert_eq!(Solution::minimum_steps("101".to_string()), 1);
}

#[test]
fn test_minimum_steps2() {
    assert_eq!(Solution::minimum_steps("100".to_string()), 2);
}

#[test]
fn test_minimum_steps3() {
    assert_eq!(Solution::minimum_steps("0111".to_string()), 0);
}
