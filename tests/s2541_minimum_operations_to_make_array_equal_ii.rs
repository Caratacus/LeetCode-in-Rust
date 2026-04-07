// Tests for Problem 2541: Minimum Operations to Make Array Equal II
// Java reference: src/test/java/g2501_2600/s2541_minimum_operations_to_make_array_equal_ii/SolutionTest.java

use leetcode_in_rust::s2541::minimum_operations_to_make_array_equal_ii::Solution;

#[test]
fn test_min_operations() {
    assert_eq!(Solution::min_operations(vec![4, 3, 1, 4], vec![1, 3, 7, 1], 3), 2);
}
#[test]
fn test_min_operations2() {
    assert_eq!(Solution::min_operations(vec![3, 8, 5, 2], vec![2, 4, 1, 6], 1), -1);
}
