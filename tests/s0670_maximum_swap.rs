// Tests for Problem 0670: Maximum Swap
// Java reference: src/test/java/g0601_0700/s0670_maximum_swap/SolutionTest.java

use leetcode_in_rust::s0670::maximum_swap::Solution;

#[test]
fn test_maximum_swap() {
    assert_eq!(Solution::maximum_swap(2736), 7236);
}

#[test]
fn test_maximum_swap2() {
    assert_eq!(Solution::maximum_swap(9973), 9973);
}
