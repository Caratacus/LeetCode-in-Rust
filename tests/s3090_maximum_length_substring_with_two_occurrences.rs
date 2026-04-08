// Tests for Problem 3090: Maximum Length Substring With Two Occurrences
// Java reference: src/test/java/g3001_3100/s3090_maximum_length_substring_with_two_occurrences/SolutionTest.java

use leetcode_in_rust::s3090::maximum_length_substring_with_two_occurrences::Solution;

#[test]
fn test_maximum_length_substring() {
    assert_eq!(Solution::maximum_length_substring("bcbbbcba".to_string()), 4);
}

#[test]
fn test_maximum_length_substring2() {
    assert_eq!(Solution::maximum_length_substring("aaaa".to_string()), 2);
}
