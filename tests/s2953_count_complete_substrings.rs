// Tests for Problem 2953: Count Complete Substrings
// Java reference: src/test/java/g2901_3000/s2953_count_complete_substrings/SolutionTest.java

use leetcode_in_rust::s2953::count_complete_substrings::Solution;

#[test]
fn test_count_complete_substrings() {
    assert_eq!(Solution::count_complete_substrings("igigee".to_string(), 2), 3);
}

#[test]
fn test_count_complete_substrings2() {
    assert_eq!(Solution::count_complete_substrings("aaabbbccc".to_string(), 3), 6);
}
