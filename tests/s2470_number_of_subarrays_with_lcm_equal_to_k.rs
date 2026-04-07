// Tests for Problem 2470: Number of Subarrays With LCM Equal to K
// Java reference: src/test/java/g2401_2500/s2470_number_of_subarrays_with_lcm_equal_to_k/SolutionTest.java

use leetcode_in_rust::s2470::number_of_subarrays_with_lcm_equal_to_k::Solution;

#[test]
fn test_subarray_lcm() {
    assert_eq!(Solution::subarray_lcm(vec![3, 6, 2, 7, 1], 6), 4);
}

#[test]
fn test_subarray_lcm2() {
    assert_eq!(Solution::subarray_lcm(vec![3], 2), 0);
}
