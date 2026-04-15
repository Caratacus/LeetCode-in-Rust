// Tests for Problem 3621: Number of Integers with Popcount Depth Equal to K I
// Java reference: src/test/java/g3601_3700/s3621_number_of_integers_with_popcount_depth_equal_to_k_i/SolutionTest.java
use leetcode_in_rust::s3621::number_of_integers_with_popcount_depth_equal_to_k_i::Solution;
#[test]
fn test_popcount_depth() { assert_eq!(Solution::popcount_depth(4i64, 1), 2i64); }
#[test]
fn test_popcount_depth2() { assert_eq!(Solution::popcount_depth(7i64, 2), 3i64); }
