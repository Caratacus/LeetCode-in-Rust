// Tests for Problem 1827: Minimum Operations to Make the Array Increasing
// Java reference: src/test/java/g1801_1900/s1827_minimum_operations_to_make_the_array_increasing/SolutionTest.java

use leetcode_in_rust::s1827::minimum_operations_to_make_the_array_increasing::Solution;

#[test]
fn test_min_operations() {
    assert_eq!(Solution::min_operations(vec![1, 1, 1]), 3);
}

#[test]
fn test_min_operations2() {
    assert_eq!(Solution::min_operations(vec![1, 5, 2, 4, 1]), 14);
}

#[test]
fn test_min_operations3() {
    assert_eq!(Solution::min_operations(vec![8]), 0);
}
