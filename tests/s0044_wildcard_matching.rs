// Tests for Problem 0044: Wildcard Matching
// Java reference: src/test/java/g0001_0100/s0044_wildcard_matching/SolutionTest.java

use leetcode_in_rust::s0044::wildcard_matching::Solution;

#[test]
fn test_is_match() {
    assert_eq!(Solution::is_match("aa".to_string(), "a".to_string()), false);
}

#[test]
fn test_is_match2() {
    assert_eq!(Solution::is_match("aa".to_string(), "*".to_string()), true);
}

#[test]
fn test_is_match3() {
    assert_eq!(Solution::is_match("cb".to_string(), "?a".to_string()), false);
}

#[test]
fn test_is_match4() {
    assert_eq!(Solution::is_match("adceb".to_string(), "*a*b".to_string()), true);
}

#[test]
fn test_is_match5() {
    assert_eq!(Solution::is_match("acdcb".to_string(), "a*c?b".to_string()), false);
}
