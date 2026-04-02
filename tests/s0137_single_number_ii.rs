// Tests for Problem 0137: Single Number II
// Java reference: src/test/java/g0121_0200/s0137_single_number_ii/SolutionTest.java

use leetcode_in_rust::s0137::single_number_ii::Solution;

#[test]
fn test_single_number() {
    assert_eq!(Solution::single_number(vec![2, 2, 3, 2]), 3);
}

#[test]
fn test_single_number2() {
    assert_eq!(Solution::single_number(vec![0, 1, 0, 1, 0, 1, 99]), 99);
}
