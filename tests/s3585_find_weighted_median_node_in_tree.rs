// Tests for Problem 3585: Find Weighted Median Node in Tree
// Java reference: src/test/java/g3501_3600/s3585_find_weighted_median_node_in_tree/SolutionTest.java
use leetcode_in_rust::s3585::find_weighted_median_node_in_tree::Solution;
#[test] fn test_find_median() { assert_eq!(Solution::find_median(2, vec![vec![0, 1, 7]], vec![vec![1, 0], vec![0, 1]]), vec![0, 1]); }
#[test] fn test_find_median2() { assert_eq!(Solution::find_median(3, vec![vec![0, 1, 2], vec![2, 0, 4]], vec![vec![0, 1], vec![2, 0], vec![1, 2]]), vec![1, 0, 2]); }
#[test] fn test_find_median3() { assert_eq!(Solution::find_median(5, vec![vec![0, 1, 2], vec![0, 2, 5], vec![1, 3, 1], vec![2, 4, 3]], vec![vec![3, 4], vec![1, 2]]), vec![2, 2]); }
