// Tests for Problem 2859: Sum of Values at Indices With K Set Bits
// Java reference: src/test/java/g2801_2900/s2859_sum_of_values_at_indices_with_k_set_bits/SolutionTest.java

use leetcode_in_rust::s2859::sum_of_values_at_indices_with_k_set_bits::Solution;

#[test]
fn test_sum_indices_with_k_set_bits() {
    assert_eq!(Solution::sum_indices_with_k_set_bits(vec![5, 10, 1, 5, 2], 1), 13);
}

#[test]
fn test_sum_indices_with_k_set_bits2() {
    assert_eq!(Solution::sum_indices_with_k_set_bits(vec![4, 3, 2, 1], 2), 1);
}
