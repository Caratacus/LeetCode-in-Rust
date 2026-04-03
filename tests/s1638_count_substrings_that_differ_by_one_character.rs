// Tests for Problem 1638: Count Substrings That Differ by One Character
// Java reference: src/test/java/g1601_1700/s1638_count_substrings_that_differ_by_one_character/SolutionTest.java

use leetcode_in_rust::s1638::count_substrings_that_differ_by_one_character::Solution;

#[test]
fn test_count_substrings() {
    assert_eq!(Solution::count_substrings("aba".to_string(), "baba".to_string()), 6);
}

#[test]
fn test_count_substrings2() {
    assert_eq!(Solution::count_substrings("ab".to_string(), "bb".to_string()), 3);
}
