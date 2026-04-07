// Tests for Problem 2000: Reverse Prefix of Word
// Java reference: src/test/java/g2001_2100/s2000_reverse_prefix_of_word/SolutionTest.java

use leetcode_in_rust::s2000::reverse_prefix_of_word::Solution;

#[test]
fn test_reverse_prefix() {
    assert_eq!(Solution::reverse_prefix(String::from("abcdefd"), 'd'), "dcbaefd");
}

#[test]
fn test_reverse_prefix2() {
    assert_eq!(Solution::reverse_prefix(String::from("xyxzxe"), 'z'), "zxyxxe");
}

#[test]
fn test_reverse_prefix3() {
    assert_eq!(Solution::reverse_prefix(String::from("abcd"), 'z'), "abcd");
}
