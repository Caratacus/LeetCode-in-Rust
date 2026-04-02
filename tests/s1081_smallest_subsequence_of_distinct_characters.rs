// Tests for Problem 1081: Smallest Subsequence of Distinct Characters
// Java reference: src/test/java/g1001_1100/s1081_smallest_subsequence_of_distinct_characters/SolutionTest.java

use leetcode_in_rust::s1081::smallest_subsequence_of_distinct_characters::Solution;

#[test]
fn test_smallest_subsequence() {
    assert_eq!(Solution::smallest_subsequence("bcabc".to_string()), "abc");
}

#[test]
fn test_smallest_subsequence2() {
    assert_eq!(Solution::smallest_subsequence("cbacdcbc".to_string()), "acdb");
}
