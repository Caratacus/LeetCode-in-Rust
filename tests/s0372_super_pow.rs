// Tests for Problem 0372: Super Pow
// Java reference: src/test/java/g0301_0400/s0372_super_pow/SolutionTest.java

use leetcode_in_rust::s0372::super_pow::Solution;

#[test]
fn test_super_pow() {
    assert_eq!(Solution::super_pow(2, vec![3]), 8);
}

#[test]
fn test_super_pow2() {
    assert_eq!(Solution::super_pow(2, vec![1, 0]), 1024);
}

#[test]
fn test_super_pow3() {
    assert_eq!(Solution::super_pow(1, vec![4, 3, 3, 8, 5, 2]), 1);
}

#[test]
fn test_super_pow4() {
    assert_eq!(Solution::super_pow(2147483647, vec![2, 0, 0]), 1198);
}
