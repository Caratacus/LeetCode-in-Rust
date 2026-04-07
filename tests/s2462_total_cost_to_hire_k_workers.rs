// Tests for Problem 2462: Total Cost to Hire K Workers
// Java reference: src/test/java/g2401_2500/s2462_total_cost_to_hire_k_workers/SolutionTest.java

use leetcode_in_rust::s2462::total_cost_to_hire_k_workers::Solution;

#[test]
fn test_total_cost() {
    assert_eq!(Solution::total_cost(vec![17, 12, 10, 2, 7, 2, 11, 20, 8], 3, 4), 11);
}

#[test]
fn test_total_cost2() {
    assert_eq!(Solution::total_cost(vec![1, 2, 4, 1], 3, 3), 4);
}
