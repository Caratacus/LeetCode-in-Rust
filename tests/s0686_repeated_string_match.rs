// Tests for Problem 0686: Repeated String Match
// Java reference: src/test/java/g0601_0700/s0686_repeated_string_match/SolutionTest.java

use leetcode_in_rust::s0686::repeated_string_match::Solution;

#[test]
fn test_repeated_string_match() {
    assert_eq!(Solution::repeated_string_match("abcd".to_string(), "cdabcdab".to_string()), 3);
}

#[test]
fn test_repeated_string_match2() {
    assert_eq!(Solution::repeated_string_match("a".to_string(), "aa".to_string()), 2);
}
