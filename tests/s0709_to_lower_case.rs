// Tests for Problem 0709: To Lower Case
// Java reference: src/test/java/g0701_0800/s0709_to_lower_case/SolutionTest.java

use leetcode_in_rust::s0709::to_lower_case::Solution;

#[test]
fn test_to_lower_case() {
    assert_eq!(Solution::to_lower_case("Hello".to_string()), "hello");
}

#[test]
fn test_to_lower_case2() {
    assert_eq!(Solution::to_lower_case("here".to_string()), "here");
}

#[test]
fn test_to_lower_case3() {
    assert_eq!(Solution::to_lower_case("LOVELY".to_string()), "lovely");
}
