// Tests for Problem 0693: Binary Number with Alternating Bits
// Java reference: src/test/java/g0601_0700/s0693_binary_number_with_alternating_bits/SolutionTest.java

use leetcode_in_rust::s0693::binary_number_with_alternating_bits::Solution;

#[test]
fn test_has_alternating_bits() {
    assert_eq!(Solution::has_alternating_bits(5), true);
}

#[test]
fn test_has_alternating_bits2() {
    assert_eq!(Solution::has_alternating_bits(7), false);
}

#[test]
fn test_has_alternating_bits3() {
    assert_eq!(Solution::has_alternating_bits(11), false);
}
