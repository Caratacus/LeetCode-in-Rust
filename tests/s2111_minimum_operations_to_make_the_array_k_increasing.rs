// Tests for Problem 2111: Minimum Operations to Make the Array K Increasing
// Java reference: src/test/java/g2101_2200/s2111_minimum_operations_to_make_the_array_k_increasing/SolutionTest.java

use leetcode_in_rust::s2111::minimum_operations_to_make_the_array_k_increasing::Solution;

#[test]
fn test_k_increasing() {
    assert_eq!(Solution::k_increasing(vec![5, 4, 3, 2, 1], 1), 4);
}

#[test]
fn test_k_increasing2() {
    assert_eq!(Solution::k_increasing(vec![4, 1, 5, 2, 6, 2], 2), 0);
}

#[test]
fn test_k_increasing3() {
    assert_eq!(Solution::k_increasing(vec![4, 1, 5, 2, 6, 2], 3), 2);
}
