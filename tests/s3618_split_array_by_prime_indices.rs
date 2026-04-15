// Tests for Problem 3618: Split Array by Prime Indices
// Java reference: src/test/java/g3601_3700/s3618_split_array_by_prime_indices/SolutionTest.java
use leetcode_in_rust::s3618::split_array_by_prime_indices::Solution;
#[test]
fn test_split_array() { assert_eq!(Solution::split_array(vec![2, 3, 4]), 1i64); }
#[test]
fn test_split_array2() { assert_eq!(Solution::split_array(vec![-1, 5, 7, 0]), 3i64); }
#[test]
fn test_split_array3() {
    assert_eq!(Solution::split_array(vec![-54818575, 801071518, 745054848, -415289833, 161564441, 706292027, 306478283, 943480367, 222076810, 992619933]), 449455001i64);
}
