// Tests for Problem 1186: Maximum Subarray Sum with One Deletion
// Java reference: src/test/java/g1101_1200/s1186_maximum_subarray_sum_with_one_deletion/SolutionTest.java

use leetcode_in_rust::s1186::maximum_subarray_sum_with_one_deletion::Solution;

#[test]
fn test_maximum_sum() {
    assert_eq!(Solution::maximum_sum(vec![1, -2, 0, 3]), 4);
}

#[test]
fn test_maximum_sum2() {
    assert_eq!(Solution::maximum_sum(vec![-1, -1, -1, -1]), -1);
}

#[test]
fn test_maximum_sum3() {
    assert_eq!(Solution::maximum_sum(vec![-1, -2, -2, -3]), -1);
}
