// Tests for Problem 3325: Count Substrings With K-Frequency Characters I
// Java reference: src/test/java/g3301_3400/s3325_count_substrings_with_k_frequency_characters_i/SolutionTest.java

use leetcode_in_rust::s3325::count_substrings_with_k_frequency_characters_i::Solution;

#[test]
fn test_number_of_substrings() {
    assert_eq!(Solution::number_of_substrings("abacb".to_string(), 2), 4);
}

#[test]
fn test_number_of_substrings2() {
    assert_eq!(Solution::number_of_substrings("abcde".to_string(), 1), 15);
}
