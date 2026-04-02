// Tests for Problem 0136: Single Number
// Java reference: src/test/java/g0121_0200/s0136_single_number/SolutionTest.java

use leetcode_in_rust::s0136::single_number::Solution;

#[test]
fn test_single_number() {
    assert_eq!(Solution::single_number(vec![2, 2, 1]), 1);
}

#[test]
fn test_single_number2() {
    assert_eq!(Solution::single_number(vec![4, 1, 2, 1, 2]), 4);
}

#[test]
fn test_single_number3() {
    assert_eq!(Solution::single_number(vec![1]), 1);
}
