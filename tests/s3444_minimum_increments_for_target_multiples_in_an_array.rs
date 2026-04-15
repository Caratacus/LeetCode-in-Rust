// Tests for Problem 3444: Minimum Increments for Target Multiples in an Array
// Java reference: src/test/java/g3401_3500/s3444_minimum_increments_for_target_multiples_in_an_array/SolutionTest.java

use leetcode_in_rust::s3444::minimum_increments_for_target_multiples_in_an_array::Solution;

#[test]
fn test_minimum_increments() {
    assert_eq!(Solution::minimum_increments(vec![1, 2, 3], vec![4]), 1);
}

#[test]
fn test_minimum_increments2() {
    assert_eq!(Solution::minimum_increments(vec![8, 4], vec![10, 5]), 2);
}

#[test]
fn test_minimum_increments3() {
    assert_eq!(Solution::minimum_increments(vec![7, 9, 10], vec![7]), 0);
}
