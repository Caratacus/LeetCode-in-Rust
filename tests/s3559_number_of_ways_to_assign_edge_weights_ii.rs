// Tests for Problem 3559: Number of Ways to Assign Edge Weights II
// Java reference: src/test/java/g3501_3600/s3559_number_of_ways_to_assign_edge_weights_ii/SolutionTest.java
use leetcode_in_rust::s3559::number_of_ways_to_assign_edge_weights_ii::Solution;
#[test] fn test_assign_edge_weights() { assert_eq!(Solution::assign_edge_weights(vec![vec![1, 2]], vec![vec![1, 1], vec![1, 2]]), vec![0, 1]); }
#[test] fn test_assign_edge_weights2() { assert_eq!(Solution::assign_edge_weights(vec![vec![1, 2], vec![1, 3], vec![3, 4], vec![3, 5]], vec![vec![1, 4], vec![3, 4], vec![2, 5]]), vec![2, 1, 4]); }
