// Tests for Problem 1702: Maximum Binary String After Change
// Java reference: src/test/java/g1701_1800/s1702_maximum_binary_string_after_change/SolutionTest.java

use leetcode_in_rust::s1702::maximum_binary_string_after_change::Solution;

#[test]
fn test_maximum_binary_string() {
    assert_eq!(Solution::maximum_binary_string("000110".to_string()), "111011".to_string());
}

#[test]
fn test_maximum_binary_string2() {
    assert_eq!(Solution::maximum_binary_string("01".to_string()), "01".to_string());
}
