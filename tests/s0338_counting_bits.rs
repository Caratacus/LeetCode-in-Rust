// Tests for Problem 0338: Counting Bits
// Java reference: src/test/java/g0301_0400/s0338_counting_bits/SolutionTest.java

use leetcode_in_rust::s0338::counting_bits::Solution;

#[test]
fn test_count_bits() {
    assert_eq!(Solution::count_bits(2), vec![0, 1, 1]);
}

#[test]
fn test_count_bits2() {
    assert_eq!(Solution::count_bits(5), vec![0, 1, 1, 2, 1, 2]);
}
