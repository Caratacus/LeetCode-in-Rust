// Tests for Problem 2817: Minimum Absolute Difference Between Elements with Constraint
// Java reference: src/test/java/g2801_2900/s2817_minimum_absolute_difference_between_elements_with_constraint/SolutionTest.java

use leetcode_in_rust::s2817::minimum_absolute_difference_between_elements_with_constraint::Solution;

#[test]
fn test_min_absolute_difference() {
    assert_eq!(Solution::min_absolute_difference(vec![4, 3, 2, 4], 2), 0);
}

#[test]
fn test_min_absolute_difference2() {
    assert_eq!(Solution::min_absolute_difference(vec![5, 3, 2, 10, 15], 1), 1);
}

#[test]
fn test_min_absolute_difference3() {
    assert_eq!(Solution::min_absolute_difference(vec![1, 2, 3, 4], 3), 3);
}
