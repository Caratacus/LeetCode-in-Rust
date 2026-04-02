// Tests for Problem 0312: Burst Balloons
// Java reference: src/test/java/g0301_0400/s0312_burst_balloons/SolutionTest.java

use leetcode_in_rust::s0312::burst_balloons::Solution;

#[test]
fn test_max_coins() {
    assert_eq!(Solution::max_coins(vec![3, 1, 5, 8]), 167);
}

#[test]
fn test_max_coins2() {
    assert_eq!(Solution::max_coins(vec![1, 5]), 10);
}
