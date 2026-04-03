// Tests for Problem 1551: Minimum Operations to Make Array Equal
// Java reference: src/test/java/g1501_1600/s1551_minimum_operations_to_make_array_equal/SolutionTest.java

use leetcode_in_rust::s1551::minimum_operations_to_make_array_equal::Solution;

#[test]
fn test_min_operations() {
    assert_eq!(Solution::min_operations(3), 2);
}

#[test]
fn test_min_operations2() {
    assert_eq!(Solution::min_operations(6), 9);
}
