// Tests for Problem 0828: Count Unique Characters of All Substrings of a Given String
// Java reference: src/test/java/g0801_0900/s0828_count_unique_characters_of_all_substrings_of_a_given_string/SolutionTest.java

use leetcode_in_rust::s0828::count_unique_characters_of_all_substrings_of_a_given_string::Solution;

#[test]
fn test_unique_letter_string() {
    assert_eq!(Solution::unique_letter_string("ABC".to_string()), 10);
}

#[test]
fn test_unique_letter_string2() {
    assert_eq!(Solution::unique_letter_string("ABA".to_string()), 8);
}

#[test]
fn test_unique_letter_string3() {
    assert_eq!(Solution::unique_letter_string("LEETCODE".to_string()), 92);
}
