// Tests for Problem 1347: Minimum Number of Steps to Make Two Strings Anagram
// Java reference: src/test/java/g1301_1400/s1347_minimum_number_of_steps_to_make_two_strings_anagram/SolutionTest.java

use leetcode_in_rust::s1347::minimum_number_of_steps_to_make_two_strings_anagram::Solution;

#[test]
fn test_min_steps() {
    assert_eq!(Solution::min_steps("bab".to_string(), "aba".to_string()), 1);
}

#[test]
fn test_min_steps2() {
    assert_eq!(Solution::min_steps("leetcode".to_string(), "practice".to_string()), 5);
}

#[test]
fn test_min_steps3() {
    assert_eq!(Solution::min_steps("anagram".to_string(), "mangaar".to_string()), 0);
}
