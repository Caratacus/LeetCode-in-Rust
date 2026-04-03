// Tests for Problem 0696: Count Binary Substrings
// Java reference: src/test/java/g0601_0700/s0696_count_binary_substrings/SolutionTest.java

use leetcode_in_rust::s0696::count_binary_substrings::Solution;

#[test]
fn test_count_binary_substrings() {
    assert_eq!(Solution::count_binary_substrings("00110011".to_string()), 6);
}

#[test]
fn test_count_binary_substrings2() {
    assert_eq!(Solution::count_binary_substrings("10101".to_string()), 4);
}
