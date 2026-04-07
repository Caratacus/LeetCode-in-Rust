// Tests for Problem 1959: Minimum Total Space Wasted With K Resizing Operations
// Java reference: src/test/java/g1901_2000/s1959_minimum_total_space_wasted_with_k_resizing_operations/SolutionTest.java

use leetcode_in_rust::s1959::minimum_total_space_wasted_with_k_resizing_operations::Solution;

#[test]
fn test_min_space_wasted_k_resizing() {
    assert_eq!(Solution::min_space_wasted_k_resizing(vec![10, 20], 0), 10);
}

#[test]
fn test_min_space_wasted_k_resizing2() {
    assert_eq!(Solution::min_space_wasted_k_resizing(vec![10, 20, 30], 1), 10);
}

#[test]
fn test_min_space_wasted_k_resizing3() {
    assert_eq!(Solution::min_space_wasted_k_resizing(vec![10, 20, 15, 30, 20], 2), 15);
}
