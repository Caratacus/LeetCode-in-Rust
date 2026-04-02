// Tests for Problem 1156: Swap for Longest Repeated Character Substring
// Java reference: src/test/java/g1101_1200/s1156_swap_for_longest_repeated_character_substring/SolutionTest.java

use leetcode_in_rust::s1156::swap_for_longest_repeated_character_substring::Solution;

#[test]
fn test_max_rep_opt1() {
    assert_eq!(Solution::max_rep_opt1("ababa".to_string()), 3);
}

#[test]
fn test_max_rep_opt2() {
    assert_eq!(Solution::max_rep_opt1("aaabaaa".to_string()), 6);
}

#[test]
fn test_max_rep_opt3() {
    assert_eq!(Solution::max_rep_opt1("aaaaa".to_string()), 5);
}
