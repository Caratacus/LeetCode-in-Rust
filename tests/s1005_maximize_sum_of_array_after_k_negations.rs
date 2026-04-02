// Tests for Problem 1005: Maximize Sum Of Array After K Negations
// Java reference: src/test/java/g1001_1100/s1005_maximize_sum_of_array_after_k_negations/SolutionTest.java

use leetcode_in_rust::s1005::maximize_sum_of_array_after_k_negations::Solution;

#[test]
fn test_largest_sum_after_k_negations() {
    assert_eq!(Solution::largest_sum_after_k_negations(vec![4, 2, 3], 1), 5);
}

#[test]
fn test_largest_sum_after_k_negations2() {
    assert_eq!(Solution::largest_sum_after_k_negations(vec![3, -1, 0, 2], 3), 6);
}

#[test]
fn test_largest_sum_after_k_negations3() {
    assert_eq!(Solution::largest_sum_after_k_negations(vec![2, -3, -1, 5, -4], 2), 13);
}
