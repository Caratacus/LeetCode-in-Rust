// Tests for Problem 1016: Binary String With Substrings Representing 1 To N
// Java reference: src/test/java/g1001_1100/s1016_binary_string_with_substrings_representing_1_to_n/SolutionTest.java

use leetcode_in_rust::s1016::binary_string_with_substrings_representing_1_to_n::Solution;

#[test]
fn test_query_string() {
    assert_eq!(Solution::query_string("0110".to_string(), 3), true);
}

#[test]
fn test_query_string2() {
    assert_eq!(Solution::query_string("0110".to_string(), 4), false);
}
