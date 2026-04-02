// Tests for Problem 0485: Max Consecutive Ones
// Java reference: src/test/java/g0401_0500/s0485_max_consecutive_ones/SolutionTest.java

use leetcode_in_rust::s0485::max_consecutive_ones::Solution;

#[test]
fn test_find_max_consecutive_ones() {
    assert_eq!(Solution::find_max_consecutive_ones(vec![1, 1, 0, 1, 1, 1]), 3);
}

#[test]
fn test_find_max_consecutive_ones2() {
    assert_eq!(Solution::find_max_consecutive_ones(vec![1, 0, 1, 1, 0, 1]), 2);
}
