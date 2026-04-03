// Tests for Problem 1754: Largest Merge Of Two Strings
// Java reference: src/test/java/g1701_1800/s1754_largest_merge_of_two_strings/SolutionTest.java

use leetcode_in_rust::s1754::largest_merge_of_two_strings::Solution;

#[test]
fn test_largest_merge() {
    assert_eq!(
        Solution::largest_merge("cabaa".to_string(), "bcaaa".to_string()),
        "cbcabaaaaa"
    );
}

#[test]
fn test_largest_merge2() {
    assert_eq!(
        Solution::largest_merge("abcabc".to_string(), "abdcaba".to_string()),
        "abdcabcabcaba"
    );
}
