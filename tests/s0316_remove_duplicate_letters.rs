// Tests for Problem 0316: Remove Duplicate Letters
// Java reference: src/test/java/g0301_0400/s0316_remove_duplicate_letters/SolutionTest.java

use leetcode_in_rust::s0316::remove_duplicate_letters::Solution;

#[test]
fn test_remove_duplicate_letters() {
    assert_eq!(Solution::remove_duplicate_letters("bcabc".to_string()), "abc");
}

#[test]
fn test_remove_duplicate_letters2() {
    assert_eq!(Solution::remove_duplicate_letters("cbacdcbc".to_string()), "acdb");
}
