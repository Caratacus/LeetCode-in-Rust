// Tests for Problem 1624: Largest Substring Between Two Equal Characters
// Java reference: src/test/java/g1601_1700/s1624_largest_substring_between_two_equal_characters/SolutionTest.java

use leetcode_in_rust::s1624::largest_substring_between_two_equal_characters::Solution;

#[test]
fn test_max_length_between_equal_characters() {
    assert_eq!(Solution::max_length_between_equal_characters("aa".to_string()), 0);
}

#[test]
fn test_max_length_between_equal_characters2() {
    assert_eq!(Solution::max_length_between_equal_characters("abca".to_string()), 2);
}

#[test]
fn test_max_length_between_equal_characters3() {
    assert_eq!(Solution::max_length_between_equal_characters("cbzxy".to_string()), -1);
}
