// Tests for Problem 0010: Regular Expression Matching
// Java reference: src/test/java/g0001_0100/s0010_regular_expression_matching/SolutionTest.java

use leetcode_in_rust::s0010::regular_expression_matching::Solution;

#[test]
fn test_is_match() {
    assert_eq!(Solution::is_match("aa".to_string(), "a".to_string()), false);
}

#[test]
fn test_is_match2() {
    assert_eq!(Solution::is_match("aa".to_string(), "a*".to_string()), true);
}

#[test]
fn test_is_match3() {
    assert_eq!(Solution::is_match("ab".to_string(), ".*".to_string()), true);
}

#[test]
fn test_is_match4() {
    assert_eq!(Solution::is_match("aab".to_string(), "c*a*b".to_string()), true);
}

#[test]
fn test_is_match5() {
    assert_eq!(Solution::is_match("mississippi".to_string(), "mis*is*p*.".to_string()), false);
}
