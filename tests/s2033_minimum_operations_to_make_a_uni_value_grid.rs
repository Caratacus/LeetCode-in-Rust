// Tests for Problem 2033: Minimum Operations to Make a Uni-Value Grid
// Java reference: src/test/java/g2001_2100/s2033_minimum_operations_to_make_a_uni_value_grid/SolutionTest.java

use leetcode_in_rust::s2033::minimum_operations_to_make_a_uni_value_grid::Solution;

#[test]
fn test_min_operations() {
    assert_eq!(Solution::min_operations(vec![vec![2, 4], vec![6, 8]], 2), 4);
}

#[test]
fn test_min_operations2() {
    assert_eq!(Solution::min_operations(vec![vec![1, 5], vec![2, 3]], 1), 5);
}

#[test]
fn test_min_operations3() {
    assert_eq!(Solution::min_operations(vec![vec![1, 2], vec![3, 4]], 2), -1);
}
