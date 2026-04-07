// Tests for Problem 1863: Sum of All Subset XOR Totals
// Java reference: src/test/java/g1801_1900/s1863_sum_of_all_subset_xor_totals/SolutionTest.java

use leetcode_in_rust::s1863::sum_of_all_subset_xor_totals::Solution;

#[test]
fn test_subset_xor_sum() {
    assert_eq!(Solution::subset_xor_sum(vec![1, 3]), 6);
}

#[test]
fn test_subset_xor_sum2() {
    assert_eq!(Solution::subset_xor_sum(vec![5, 1, 6]), 28);
}

#[test]
fn test_subset_xor_sum3() {
    assert_eq!(Solution::subset_xor_sum(vec![3, 4, 5, 6, 7, 8]), 480);
}
