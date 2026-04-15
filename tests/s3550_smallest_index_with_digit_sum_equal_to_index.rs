// Tests for Problem 3550: Smallest Index with Digit Sum Equal to Index
// Java reference: src/test/java/g3501_3600/s3550_smallest_index_with_digit_sum_equal_to_index/SolutionTest.java

use leetcode_in_rust::s3550::smallest_index_with_digit_sum_equal_to_index::Solution;

#[test]
fn test_smallest_index() { assert_eq!(Solution::smallest_index(vec![1, 3, 2]), 2); }
#[test]
fn test_smallest_index2() { assert_eq!(Solution::smallest_index(vec![1, 10, 11]), 1); }
#[test]
fn test_smallest_index3() { assert_eq!(Solution::smallest_index(vec![1, 2, 3]), -1); }
