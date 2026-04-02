// Tests for Problem 0467: Unique Substrings in Wraparound String
// Java reference: src/test/java/g0401_0500/s0467_unique_substrings_in_wraparound_string/SolutionTest.java

use leetcode_in_rust::s0467::unique_substrings_in_wraparound_string::Solution;

#[test]
fn test_find_substring_in_wrapround_string() {
    assert_eq!(Solution::find_substring_in_wrapround_string("a".to_string()), 1);
}

#[test]
fn test_find_substring_in_wrapround_string2() {
    assert_eq!(Solution::find_substring_in_wrapround_string("cac".to_string()), 2);
}

#[test]
fn test_find_substring_in_wrapround_string3() {
    assert_eq!(Solution::find_substring_in_wrapround_string("zab".to_string()), 6);
}
