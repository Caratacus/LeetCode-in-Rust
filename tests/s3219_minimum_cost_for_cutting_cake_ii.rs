// Tests for Problem 3219: Minimum Cost for Cutting Cake II
// Java reference: src/test/java/g3201_3300/s3219_minimum_cost_for_cutting_cake_ii/SolutionTest.java

use leetcode_in_rust::s3219::minimum_cost_for_cutting_cake_ii::Solution;

#[test]
fn test_minimum_cost() {
    assert_eq!(Solution::minimum_cost(3, 2, vec![1, 3], vec![5]), 13);
}

#[test]
fn test_minimum_cost2() {
    assert_eq!(Solution::minimum_cost(2, 2, vec![7], vec![4]), 15);
}
