// Tests for Problem 0069: Sqrt(x)
// Java reference: src/test/java/g0001_0100/s0069_sqrtx/SolutionTest.java

use leetcode_in_rust::s0069::sqrtx::Solution;

#[test]
fn test_my_sqrt() {
    assert_eq!(Solution::my_sqrt(4), 2);
}

#[test]
fn test_my_sqrt2() {
    assert_eq!(Solution::my_sqrt(8), 2);
}

#[test]
fn test_my_sqrt3() {
    assert_eq!(Solution::my_sqrt(0), 0);
}
