// Tests for Problem 2186: Minimum Number of Steps to Make Two Strings Anagram II
// Java reference: src/test/java/g2101_2200/s2186_minimum_number_of_steps_to_make_two_strings_anagram_ii/SolutionTest.java

use leetcode_in_rust::s2186::minimum_number_of_steps_to_make_two_strings_anagram_ii::Solution;

#[test]
fn test_min_steps() {
    assert_eq!(Solution::min_steps("leetcode".to_string(), "coats".to_string()), 7);
}

#[test]
fn test_min_steps2() {
    assert_eq!(Solution::min_steps("night".to_string(), "thing".to_string()), 0);
}
