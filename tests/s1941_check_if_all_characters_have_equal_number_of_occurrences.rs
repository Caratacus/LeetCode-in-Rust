// Tests for Problem 1941: Check if All Characters Have Equal Number of Occurrences
// Java reference: src/test/java/g1901_2000/s1941_check_if_all_characters_have_equal_number_of_occurrences/SolutionTest.java

use leetcode_in_rust::s1941::check_if_all_characters_have_equal_number_of_occurrences::Solution;

#[test]
fn test_are_occurrences_equal() {
    assert_eq!(Solution::are_occurrences_equal("abacbc".to_string()), true);
}

#[test]
fn test_are_occurrences_equal2() {
    assert_eq!(Solution::are_occurrences_equal("aaabb".to_string()), false);
}
