// Tests for Problem 2781: Length of the Longest Valid Substring
// Java reference: src/test/java/g2701_2800/s2781_length_of_the_longest_valid_substring/SolutionTest.java

use leetcode_in_rust::s2781::length_of_the_longest_valid_substring::Solution;

#[test]
fn test_longest_valid_substring() {
    assert_eq!(
        Solution::longest_valid_substring("cbaaaabc".to_string(), vec!["aaa".to_string(), "cb".to_string()]),
        4
    );
}

#[test]
fn test_longest_valid_substring2() {
    assert_eq!(
        Solution::longest_valid_substring("leetcode".to_string(), vec!["de".to_string(), "le".to_string(), "e".to_string()]),
        4
    );
}
