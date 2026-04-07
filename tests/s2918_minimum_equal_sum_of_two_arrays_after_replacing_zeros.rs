// Tests for Problem 2918: Minimum Equal Sum of Two Arrays After Replacing Zeros
// Java reference: src/test/java/g2901_3000/s2918_minimum_equal_sum_of_two_arrays_after_replacing_zeros/SolutionTest.java

use leetcode_in_rust::s2918::minimum_equal_sum_of_two_arrays_after_replacing_zeros::Solution;

#[test]
fn test_min_sum() {
    assert_eq!(Solution::min_sum(vec![3, 2, 0, 1, 0], vec![6, 5, 0]), 12);
}

#[test]
fn test_min_sum2() {
    assert_eq!(Solution::min_sum(vec![2, 0, 2, 0], vec![1, 4]), -1);
}
