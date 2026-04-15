// Tests for Problem 3407: Substring Matching Pattern
// Java reference: src/test/java/g3401_3500/s3407_substring_matching_pattern/SolutionTest.java

use leetcode_in_rust::s3407::substring_matching_pattern::Solution;

#[test]
fn test_has_match() {
    assert_eq!(Solution::has_match("leetcode".to_string(), "ee*e".to_string()), true);
}

#[test]
fn test_has_match2() {
    assert_eq!(Solution::has_match("car".to_string(), "c*v".to_string()), false);
}

#[test]
fn test_has_match3() {
    assert_eq!(Solution::has_match("luck".to_string(), "u*".to_string()), true);
}
