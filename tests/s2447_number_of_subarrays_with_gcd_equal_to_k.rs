// Tests for Problem 2447: Number of Subarrays With GCD Equal to K
// Java reference: src/test/java/g2401_2500/s2447_number_of_subarrays_with_gcd_equal_to_k/SolutionTest.java

use leetcode_in_rust::s2447::number_of_subarrays_with_gcd_equal_to_k::Solution;

#[test]
fn test_subarray_gcd() {
    assert_eq!(Solution::subarray_gcd(vec![9, 3, 1, 2, 6, 3], 3), 4);
}

#[test]
fn test_subarray_gcd2() {
    assert_eq!(Solution::subarray_gcd(vec![4], 7), 0);
}
