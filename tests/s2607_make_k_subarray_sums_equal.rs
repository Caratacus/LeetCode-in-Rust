// Tests for Problem 2607: Make K Subarray Sums Equal
// Java reference: src/test/java/g2601_2700/s2607_make_k_subarray_sums_equal/SolutionTest.java

use leetcode_in_rust::s2607::make_k_subarray_sums_equal::Solution;

#[test]
fn test_make_sub_k_sum_equal() {
    assert_eq!(Solution::make_sub_k_sum_equal(vec![1, 4, 1, 3], 2), 1);
}

#[test]
fn test_make_sub_k_sum_equal2() {
    assert_eq!(Solution::make_sub_k_sum_equal(vec![2, 5, 5, 7], 3), 5);
}
