// Tests for Problem 3649: Number of Perfect Pairs
// Java reference: src/test/java/g3601_3700/s3649_number_of_perfect_pairs/SolutionTest.java
use leetcode_in_rust::s3649::number_of_perfect_pairs::Solution;
#[test]
fn test_perfect_pairs() { assert_eq!(Solution::perfect_pairs(vec![0, 1, 2, 3]), 2i64); }
#[test]
fn test_perfect_pairs2() { assert_eq!(Solution::perfect_pairs(vec![-3, 2, -1, 4]), 4i64); }
#[test]
fn test_perfect_pairs3() { assert_eq!(Solution::perfect_pairs(vec![1, 10, 100, 1000]), 0i64); }
