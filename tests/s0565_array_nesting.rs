// Tests for Problem 0565: Array Nesting
// Java reference: src/test/java/g0501_0600/s0565_array_nesting/SolutionTest.java

use leetcode_in_rust::s0565::array_nesting::Solution;

#[test]
fn test_array_nesting() {
    assert_eq!(Solution::array_nesting(vec![5, 4, 0, 3, 1, 6, 2]), 4);
}

#[test]
fn test_array_nesting2() {
    assert_eq!(Solution::array_nesting(vec![0, 1, 2]), 1);
}
