// Tests for Problem 2653: Sliding Subarray Beauty
// Java reference: src/test/java/g2601_2700/s2653_sliding_subarray_beauty/SolutionTest.java

use leetcode_in_rust::s2653::sliding_subarray_beauty::Solution;

#[test]
fn test_get_subarray_beauty() {
    assert_eq!(
        Solution::get_subarray_beauty(vec![1, -1, -3, -2, 3], 3, 2),
        vec![-1, -2, -2]
    );
}

#[test]
fn test_get_subarray_beauty2() {
    assert_eq!(
        Solution::get_subarray_beauty(vec![-1, -2, -3, -4, -5], 2, 2),
        vec![-1, -2, -3, -4]
    );
}

#[test]
fn test_get_subarray_beauty3() {
    assert_eq!(
        Solution::get_subarray_beauty(vec![-3, 1, 2, -3, 0, -3], 2, 1),
        vec![-3, 0, -3, -3, -3]
    );
}
