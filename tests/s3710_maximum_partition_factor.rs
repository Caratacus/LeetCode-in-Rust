// Tests for Problem 3710: Maximum Partition Factor
// Java reference: src/test/java/g3701_3800/s3710_maximum_partition_factor/SolutionTest.java
use leetcode_in_rust::s3710::maximum_partition_factor::Solution;
#[test]
fn test_max_partition_factor() { assert_eq!(Solution::max_partition_factor(vec![vec![0, 0], vec![0, 2], vec![2, 0], vec![2, 2]]), 4); }
#[test]
fn test_max_partition_factor2() { assert_eq!(Solution::max_partition_factor(vec![vec![0, 0], vec![0, 1], vec![10, 0]]), 11); }
