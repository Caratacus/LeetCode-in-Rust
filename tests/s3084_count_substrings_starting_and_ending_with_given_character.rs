// Tests for Problem 3084: Count Substrings Starting and Ending With Given Character
// Java reference: src/test/java/g3001_3100/s3084_count_substrings_starting_and_ending_with_given_character/SolutionTest.java

use leetcode_in_rust::s3084::count_substrings_starting_and_ending_with_given_character::Solution;

#[test]
fn test_count_substrings() {
    assert_eq!(Solution::count_substrings("abada".to_string(), 'a'), 6);
}

#[test]
fn test_count_substrings2() {
    assert_eq!(Solution::count_substrings("zzz".to_string(), 'z'), 6);
}
