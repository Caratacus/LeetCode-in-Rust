// Tests for Problem 2680: Maximum OR
// Java reference: src/test/java/g2601_2700/s2680_maximum_or/SolutionTest.java

use leetcode_in_rust::s2680::maximum_or::Solution;

#[test]
fn test_maximum_or() {
    assert_eq!(Solution::maximum_or(vec![12, 9], 1), 30);
}

#[test]
fn test_maximum_or2() {
    assert_eq!(Solution::maximum_or(vec![8, 1, 2], 2), 35);
}
