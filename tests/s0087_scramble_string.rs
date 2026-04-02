// Tests for Problem 0087: Scramble String
// Java reference: src/test/java/g0001_0100/s0087_scramble_string/SolutionTest.java

use leetcode_in_rust::s0087::scramble_string::Solution;

#[test]
fn test_is_scramble() {
    assert_eq!(Solution::is_scramble("great".to_string(), "rgeat".to_string()), true);
}

#[test]
fn test_is_scramble2() {
    assert_eq!(Solution::is_scramble("abcde".to_string(), "caebd".to_string()), false);
}

#[test]
fn test_is_scramble3() {
    assert_eq!(Solution::is_scramble("a".to_string(), "a".to_string()), true);
}
