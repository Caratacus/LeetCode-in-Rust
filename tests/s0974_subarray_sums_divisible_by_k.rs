// Tests for Problem 0974: Subarray Sums Divisible by K
// Java reference: src/test/java/g0901_1000/s0974_subarray_sums_divisible_by_k/SolutionTest.java

use leetcode_in_rust::s0974::subarray_sums_divisible_by_k::Solution;

#[test]
fn test_subarrays_div_by_k() {
    assert_eq!(Solution::subarrays_div_by_k(vec![4, 5, 0, -2, -3, 1], 5), 7);
}

#[test]
fn test_subarrays_div_by_k2() {
    assert_eq!(Solution::subarrays_div_by_k(vec![5], 9), 0);
}
