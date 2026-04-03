// Tests for Problem 0698: Partition to K Equal Sum Subsets
// Java reference: src/test/java/g0601_0700/s0698_partition_to_k_equal_sum_subsets/SolutionTest.java

use leetcode_in_rust::s0698::partition_to_k_equal_sum_subsets::Solution;

#[test]
fn test_can_partition_k_subsets() {
    assert_eq!(Solution::can_partition_k_subsets(vec![4, 3, 2, 3, 5, 2, 1], 4), true);
}

#[test]
fn test_can_partition_k_subsets2() {
    assert_eq!(Solution::can_partition_k_subsets(vec![1, 2, 3, 4], 3), false);
}
