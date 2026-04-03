// Tests for Problem 1593: Split a String Into the Max Number of Unique Substrings
// Java reference: src/test/java/g1501_1600/s1593_split_a_string_into_the_max_number_of_unique_substrings/SolutionTest.java

use leetcode_in_rust::s1593::split_a_string_into_the_max_number_of_unique_substrings::Solution;

#[test]
fn test_max_unique_split() {
    assert_eq!(Solution::max_unique_split("ababccc".to_string()), 5);
}

#[test]
fn test_max_unique_split2() {
    assert_eq!(Solution::max_unique_split("aba".to_string()), 2);
}

#[test]
fn test_max_unique_split3() {
    assert_eq!(Solution::max_unique_split("aa".to_string()), 1);
}
