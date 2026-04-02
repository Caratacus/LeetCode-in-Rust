// Tests for Problem 0857: Minimum Cost to Hire K Workers
// Java reference: src/test/java/g0801_0900/s0857_minimum_cost_to_hire_k_workers/SolutionTest.java

use leetcode_in_rust::s0857::minimum_cost_to_hire_k_workers::Solution;

#[test]
fn test_mincost_to_hire_workers() {
    let result = Solution::mincost_to_hire_workers(vec![10, 20, 5], vec![70, 50, 30], 2);
    assert!((result - 105.0).abs() < 1e-5);
}

#[test]
fn test_mincost_to_hire_workers2() {
    let result = Solution::mincost_to_hire_workers(vec![3, 1, 10, 10, 1], vec![4, 8, 2, 2, 7], 3);
    assert!((result - 30.66667).abs() < 1e-5);
}
