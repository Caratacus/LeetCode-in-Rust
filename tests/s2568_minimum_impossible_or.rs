// Tests for Problem 2568: Minimum Impossible OR
// Java reference: src/test/java/g2501_2600/s2568_minimum_impossible_or/SolutionTest.java

use leetcode_in_rust::s2568::minimum_impossible_or::Solution;

#[test]
fn test_min_impossible_or() {
    assert_eq!(Solution::min_impossible_or(vec![2, 1]), 4);
}

#[test]
fn test_min_impossible_or2() {
    assert_eq!(Solution::min_impossible_or(vec![5, 3, 2]), 1);
}
