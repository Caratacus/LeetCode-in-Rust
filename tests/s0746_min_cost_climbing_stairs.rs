// Tests for Problem 0746: Min Cost Climbing Stairs
// Java reference: src/test/java/g0701_0800/s0746_min_cost_climbing_stairs/SolutionTest.java

use leetcode_in_rust::s0746::min_cost_climbing_stairs::Solution;

#[test]
fn test_min_cost_climbing_stairs() {
    assert_eq!(Solution::min_cost_climbing_stairs(vec![10, 15, 20]), 15);
}

#[test]
fn test_min_cost_climbing_stairs2() {
    assert_eq!(
        Solution::min_cost_climbing_stairs(vec![1, 100, 1, 1, 1, 100, 1, 1, 100, 1]),
        6
    );
}
