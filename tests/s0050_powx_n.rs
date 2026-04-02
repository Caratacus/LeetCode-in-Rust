// Tests for Problem 0050: Pow(x, n)
// Java reference: src/test/java/g0001_0100/s0050_powx_n/SolutionTest.java

use leetcode_in_rust::s0050::powx_n::Solution;

#[test]
fn test_my_pow() {
    let result = Solution::my_pow(2.00000, 10);
    assert!((result - 1024.0).abs() < 1e-5);
}

#[test]
fn test_my_pow2() {
    let result = Solution::my_pow(2.10000, 3);
    assert!((result - 9.261).abs() < 1e-5);
}

#[test]
fn test_my_pow3() {
    let result = Solution::my_pow(2.00000, -2);
    assert!((result - 0.25).abs() < 1e-5);
}
