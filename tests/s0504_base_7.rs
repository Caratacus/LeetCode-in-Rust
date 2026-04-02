// Tests for Problem 0504: Base 7
// Java reference: src/test/java/g0501_0600/s0504_base_7/SolutionTest.java

use leetcode_in_rust::s0504::base_7::Solution;

#[test]
fn test_convert_to_base7() {
    assert_eq!(Solution::convert_to_base7(100), "202");
}

#[test]
fn test_convert_to_base72() {
    assert_eq!(Solution::convert_to_base7(-7), "-10");
}
