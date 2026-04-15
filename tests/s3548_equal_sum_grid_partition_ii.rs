// Tests for Problem 3548: Equal Sum Grid Partition II
// Java reference: src/test/java/g3501_3600/s3548_equal_sum_grid_partition_ii/SolutionTest.java

use leetcode_in_rust::s3548::equal_sum_grid_partition_ii::Solution;

#[test]
fn test_can_partition_grid() { assert_eq!(Solution::can_partition_grid(vec![vec![1, 4], vec![2, 3]]), true); }
#[test]
fn test_can_partition_grid2() { assert_eq!(Solution::can_partition_grid(vec![vec![1, 2], vec![3, 4]]), true); }
#[test]
fn test_can_partition_grid3() { assert_eq!(Solution::can_partition_grid(vec![vec![1, 2, 4], vec![2, 3, 5]]), false); }
#[test]
fn test_can_partition_grid5() { assert_eq!(Solution::can_partition_grid(vec![vec![1]]), false); }
