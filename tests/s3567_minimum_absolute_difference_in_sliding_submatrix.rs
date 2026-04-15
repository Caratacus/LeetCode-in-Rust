// Tests for Problem 3567: Minimum Absolute Difference in Sliding Submatrix
// Java reference: src/test/java/g3501_3600/s3567_minimum_absolute_difference_in_sliding_submatrix/SolutionTest.java
use leetcode_in_rust::s3567::minimum_absolute_difference_in_sliding_submatrix::Solution;
#[test] fn test_min_abs_diff() { assert_eq!(Solution::min_abs_diff(vec![vec![1, 8], vec![3, -2]], 2), vec![vec![2]]); }
#[test] fn test_min_abs_diff2() { assert_eq!(Solution::min_abs_diff(vec![vec![3, -1]], 1), vec![vec![0, 0]]); }
#[test] fn test_min_abs_diff3() { assert_eq!(Solution::min_abs_diff(vec![vec![1, -2, 3], vec![2, 3, 5]], 2), vec![vec![1, 2]]); }
