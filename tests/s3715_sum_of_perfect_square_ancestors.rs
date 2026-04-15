// Tests for Problem 3715: Sum of Perfect Square Ancestors
// Java reference: src/test/java/g3701_3800/s3715_sum_of_perfect_square_ancestors/SolutionTest.java
use leetcode_in_rust::s3715::sum_of_perfect_square_ancestors::Solution;
#[test]
fn test_sum_of_ancestors() { assert_eq!(Solution::sum_of_ancestors(3, vec![vec![0, 1], vec![1, 2]], vec![2, 8, 2]), 3i64); }
#[test]
fn test_sum_of_ancestors2() { assert_eq!(Solution::sum_of_ancestors(3, vec![vec![0, 1], vec![0, 2]], vec![1, 2, 4]), 1i64); }
#[test]
fn test_sum_of_ancestors3() { assert_eq!(Solution::sum_of_ancestors(4, vec![vec![0, 1], vec![0, 2], vec![1, 3]], vec![1, 2, 9, 4]), 2i64); }
