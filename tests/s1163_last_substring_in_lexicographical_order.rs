// Tests for Problem 1163: Last Substring in Lexicographical Order
// Java reference: src/test/java/g1101_1200/s1163_last_substring_in_lexicographical_order/SolutionTest.java

use leetcode_in_rust::s1163::last_substring_in_lexicographical_order::Solution;

#[test]
fn test_last_substring() {
    assert_eq!(Solution::last_substring("abab".to_string()), "bab");
}

#[test]
fn test_last_substring2() {
    assert_eq!(Solution::last_substring("leetcode".to_string()), "tcode");
}
