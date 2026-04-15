// Tests for Problem 3613: Minimize Maximum Component Cost
// Java reference: src/test/java/g3601_3700/s3613_minimize_maximum_component_cost/SolutionTest.java
use leetcode_in_rust::s3613::minimize_maximum_component_cost::Solution;
#[test]
fn test_min_cost() {
    assert_eq!(Solution::min_cost(5, vec![vec![0, 1, 4], vec![1, 2, 3], vec![1, 3, 2], vec![3, 4, 6]], 2), 4);
}
#[test]
fn test_min_cost2() {
    assert_eq!(Solution::min_cost(4, vec![vec![0, 1, 5], vec![1, 2, 5], vec![2, 3, 5]], 1), 5);
}
