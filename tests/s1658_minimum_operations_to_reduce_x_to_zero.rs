// Tests for Problem 1658: Minimum Operations to Reduce X to Zero
// Java reference: src/test/java/g1601_1700/s1658_minimum_operations_to_reduce_x_to_zero/SolutionTest.java

use leetcode_in_rust::s1658::minimum_operations_to_reduce_x_to_zero::Solution;

#[test]
fn test_min_operations() {
    assert_eq!(Solution::min_operations(vec![1, 1, 4, 2, 3], 5), 2);
}

#[test]
fn test_min_operations2() {
    assert_eq!(Solution::min_operations(vec![5, 6, 7, 8, 9], 4), -1);
}

#[test]
fn test_min_operations3() {
    assert_eq!(Solution::min_operations(vec![3, 2, 20, 1, 1, 3], 10), 5);
}
