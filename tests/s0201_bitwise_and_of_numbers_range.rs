// Tests for Problem 0201: Bitwise AND of Numbers Range
// Java reference: src/test/java/g0201_0300/s0201_bitwise_and_of_numbers_range/SolutionTest.java

use leetcode_in_rust::s0201::bitwise_and_of_numbers_range::Solution;

#[test]
fn test_range_bitwise_and() {
    assert_eq!(Solution::range_bitwise_and(5, 7), 4);
}

#[test]
fn test_range_bitwise_and2() {
    assert_eq!(Solution::range_bitwise_and(0, 0), 0);
}

#[test]
fn test_range_bitwise_and3() {
    assert_eq!(Solution::range_bitwise_and(1, 2147483647), 0);
}
