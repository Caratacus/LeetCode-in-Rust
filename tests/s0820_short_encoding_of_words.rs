// Tests for Problem 0820: Short Encoding of Words
// Java reference: src/test/java/g0701_0800/s0820_short_encoding_of_words/SolutionTest.java

use leetcode_in_rust::s0820::short_encoding_of_words::Solution;

#[test]
fn test_minimum_length_encoding() {
    assert_eq!(
        Solution::minimum_length_encoding(vec!["time".to_string(), "me".to_string(), "bell".to_string()]),
        10
    );
}

#[test]
fn test_minimum_length_encoding2() {
    assert_eq!(
        Solution::minimum_length_encoding(vec!["t".to_string()]),
        2
    );
}
