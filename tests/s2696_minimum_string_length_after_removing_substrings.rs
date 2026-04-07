// Tests for Problem 2696: Minimum String Length After Removing Substrings
// Java reference: src/test/java/g2601_2700/s2696_minimum_string_length_after_removing_substrings/SolutionTest.java

use leetcode_in_rust::s2696::minimum_string_length_after_removing_substrings::Solution;

#[test]
fn test_min_length() {
    assert_eq!(Solution::min_length("ABFCACDB".to_string()), 2);
}

#[test]
fn test_min_length2() {
    assert_eq!(Solution::min_length("ACBBD".to_string()), 5);
}
