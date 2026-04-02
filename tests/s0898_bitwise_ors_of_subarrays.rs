// Tests for Problem 0898: Bitwise ORs of Subarrays
// Java reference: src/test/java/g0801_0900/s0898_bitwise_ors_of_subarrays/SolutionTest.java

use leetcode_in_rust::s0898::bitwise_ors_of_subarrays::Solution;

#[test]
fn test_subarray_bitwise_o_rs() {
    assert_eq!(Solution::subarray_bitwise_o_rs(vec![0]), 1);
}

#[test]
fn test_subarray_bitwise_o_rs2() {
    assert_eq!(Solution::subarray_bitwise_o_rs(vec![1, 1, 2]), 3);
}

#[test]
fn test_subarray_bitwise_o_rs3() {
    assert_eq!(Solution::subarray_bitwise_o_rs(vec![1, 2, 4]), 6);
}
