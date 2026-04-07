// Tests for Problem 2009: Minimum Number of Operations to Make Array Continuous
// Java reference: src/test/java/g2001_2100/s2009_minimum_number_of_operations_to_make_array_continuous/SolutionTest.java

use leetcode_in_rust::s2009::minimum_number_of_operations_to_make_array_continuous::Solution;

#[test]
fn test_min_operations() {
    assert_eq!(Solution::min_operations(vec![4, 2, 5, 3]), 0);
}

#[test]
fn test_min_operations2() {
    assert_eq!(Solution::min_operations(vec![1, 2, 3, 5, 6]), 1);
}

#[test]
fn test_min_operations3() {
    assert_eq!(Solution::min_operations(vec![1, 10, 100, 1000]), 3);
}
