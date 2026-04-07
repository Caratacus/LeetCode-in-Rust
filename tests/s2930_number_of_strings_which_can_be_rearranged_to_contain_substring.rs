// Tests for Problem 2930: Number of Strings Which Can Be Rearranged to Contain Substring
// Java reference: src/test/java/g2901_3000/s2930_number_of_strings_which_can_be_rearranged_to_contain_substring/SolutionTest.java

use leetcode_in_rust::s2930::number_of_strings_which_can_be_rearranged_to_contain_substring::Solution;

#[test]
fn test_string_count() {
    assert_eq!(Solution::string_count(4), 12);
}

#[test]
fn test_string_count2() {
    assert_eq!(Solution::string_count(10), 83943898);
}
