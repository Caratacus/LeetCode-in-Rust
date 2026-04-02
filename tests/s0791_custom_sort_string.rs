// Tests for Problem 0791: Custom Sort String
// Java reference: src/test/java/g0701_0800/s0791_custom_sort_string/SolutionTest.java

use leetcode_in_rust::s0791::custom_sort_string::Solution;

#[test]
fn test_custom_sort_string() {
    assert_eq!(Solution::custom_sort_string("cba".to_string(), "abcd".to_string()), "cbad");
}

#[test]
fn test_custom_sort_string2() {
    assert_eq!(Solution::custom_sort_string("cbafg".to_string(), "abcd".to_string()), "cbad");
}
