// Tests for Problem 3593: Minimum Increments to Equalize Leaf Paths
// Java reference: src/test/java/g3501_3600/s3593_minimum_increments_to_equalize_leaf_paths/SolutionTest.java
use leetcode_in_rust::s3593::minimum_increments_to_equalize_leaf_paths::Solution;
#[test] fn test_min_increase() { assert_eq!(Solution::min_increase(3, vec![vec![0, 1], vec![0, 2]], vec![2, 1, 3]), 1); }
#[test] fn test_min_increase2() { assert_eq!(Solution::min_increase(3, vec![vec![0, 1], vec![1, 2]], vec![5, 1, 4]), 0); }
#[test] fn test_min_increase3() { assert_eq!(Solution::min_increase(5, vec![vec![0, 4], vec![0, 1], vec![1, 2], vec![1, 3]], vec![3, 4, 1, 1, 7]), 1); }
