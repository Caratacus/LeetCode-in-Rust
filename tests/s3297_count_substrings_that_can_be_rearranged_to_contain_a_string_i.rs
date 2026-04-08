// Tests for Problem 3297: Count Substrings That Can Be Rearranged to Contain a String I
// Java reference: src/test/java/g3201_3300/s3297_count_substrings_that_can_be_rearranged_to_contain_a_string_i/SolutionTest.java

use leetcode_in_rust::s3297::count_substrings_that_can_be_rearranged_to_contain_a_string_i::Solution;

#[test]
fn test_valid_substring_count() {
    assert_eq!(Solution::valid_substring_count("bcca".to_string(), "abc".to_string()), 1);
}

#[test]
fn test_valid_substring_count2() {
    assert_eq!(Solution::valid_substring_count("abcabc".to_string(), "abc".to_string()), 10);
}

#[test]
fn test_valid_substring_count3() {
    assert_eq!(Solution::valid_substring_count("abcabc".to_string(), "aaabc".to_string()), 0);
}
