// Tests for Problem 3512: Minimum Operations to Make Array Sum Divisible by K
// Java reference: src/test/java/g3501_3600/s3512_minimum_operations_to_make_array_sum_divisible_by_k/SolutionTest.java

use leetcode_in_rust::s3512::minimum_operations_to_make_array_sum_divisible_by_k::Solution;

#[test]
fn test_min_operations() {
    assert_eq!(Solution::min_operations(vec![3, 9, 7], 5), 4);
}

#[test]
fn test_min_operations2() {
    assert_eq!(Solution::min_operations(vec![4, 1, 3], 4), 0);
}

#[test]
fn test_min_operations3() {
    assert_eq!(Solution::min_operations(vec![3, 2], 6), 5);
}
