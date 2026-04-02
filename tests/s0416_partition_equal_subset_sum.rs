// Tests for Problem 0416: Partition Equal Subset Sum
// Java reference: src/test/java/g0401_0500/s0416_partition_equal_subset_sum/SolutionTest.java

use leetcode_in_rust::s0416::partition_equal_subset_sum::Solution;

#[test]
fn test_can_partition() {
    assert_eq!(Solution::can_partition(vec![1, 5, 11, 5]), true);
}

#[test]
fn test_can_partition2() {
    assert_eq!(Solution::can_partition(vec![1, 2, 3, 5]), false);
}
