// Tests for Problem 3218: Minimum Cost for Cutting Cake I
// Java reference: src/test/java/g3201_3300/s3218_minimum_cost_for_cutting_cake_i/SolutionTest.java

use leetcode_in_rust::s3218::minimum_cost_for_cutting_cake_i::Solution;

#[test]
fn test_minimum_cost() {
    assert_eq!(Solution::minimum_cost(3, 2, vec![1, 3], vec![5]), 13);
}

#[test]
fn test_minimum_cost2() {
    assert_eq!(Solution::minimum_cost(2, 2, vec![7], vec![4]), 15);
}
