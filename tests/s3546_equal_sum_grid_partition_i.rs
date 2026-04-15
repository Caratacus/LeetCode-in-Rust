// Tests for Problem 3546: Equal Sum Grid Partition I
// Java reference: src/test/java/g3501_3600/s3546_equal_sum_grid_partition_i/SolutionTest.java

use leetcode_in_rust::s3546::equal_sum_grid_partition_i::Solution;

#[test]
fn test_can_partition_grid() { assert_eq!(Solution::can_partition_grid(vec![vec![1, 4], vec![2, 3]]), true); }
#[test]
fn test_can_partition_grid2() { assert_eq!(Solution::can_partition_grid(vec![vec![1, 3], vec![2, 4]]), false); }
#[test]
fn test_can_partition_grid3() { assert_eq!(Solution::can_partition_grid(vec![vec![1]]), false); }
