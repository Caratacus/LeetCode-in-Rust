// Tests for Problem 0879: Profitable Schemes
// Java reference: src/test/java/g0801_0900/s0879_profitable_schemes/SolutionTest.java

use leetcode_in_rust::s0879::profitable_schemes::Solution;

#[test]
fn test_profitable_schemes() {
    assert_eq!(Solution::profitable_schemes(5, 3, vec![2, 2], vec![2, 3]), 2);
}

#[test]
fn test_profitable_schemes2() {
    assert_eq!(Solution::profitable_schemes(10, 5, vec![2, 3, 5], vec![6, 7, 8]), 7);
}
