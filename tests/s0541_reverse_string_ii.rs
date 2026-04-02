// Tests for Problem 0541: Reverse String II
// Java reference: src/test/java/g0501_0600/s0541_reverse_string_ii/SolutionTest.java

use leetcode_in_rust::s0541::reverse_string_ii::Solution;

#[test]
fn test_reverse_str() {
    assert_eq!(Solution::reverse_str("abcdefg".to_string(), 2), "bacdfeg");
}

#[test]
fn test_reverse_str2() {
    assert_eq!(Solution::reverse_str("abcd".to_string(), 2), "bacd");
}
