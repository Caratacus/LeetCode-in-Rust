// Tests for Problem 2869: Minimum Operations to Collect Elements
// Java reference: src/test/java/g2801_2900/s2869_minimum_operations_to_collect_elements/SolutionTest.java

use leetcode_in_rust::s2869::minimum_operations_to_collect_elements::Solution;

#[test]
fn test_min_operations() {
    assert_eq!(Solution::min_operations(vec![3, 1, 5, 4, 2], 2), 4);
}

#[test]
fn test_min_operations2() {
    assert_eq!(Solution::min_operations(vec![3, 1, 5, 4, 2], 5), 5);
}

#[test]
fn test_min_operations3() {
    assert_eq!(Solution::min_operations(vec![3, 2, 5, 3, 1], 3), 4);
}
