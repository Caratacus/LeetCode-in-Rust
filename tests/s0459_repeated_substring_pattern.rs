// Tests for Problem 0459: Repeated Substring Pattern
// Java reference: src/test/java/g0401_0500/s0459_repeated_substring_pattern/SolutionTest.java

use leetcode_in_rust::s0459::repeated_substring_pattern::Solution;

#[test]
fn test_repeated_substring_pattern() {
    assert_eq!(Solution::repeated_substring_pattern("abab".to_string()), true);
}

#[test]
fn test_repeated_substring_pattern2() {
    assert_eq!(Solution::repeated_substring_pattern("aba".to_string()), false);
}

#[test]
fn test_repeated_substring_pattern3() {
    assert_eq!(Solution::repeated_substring_pattern("abcabcabcabc".to_string()), true);
}
