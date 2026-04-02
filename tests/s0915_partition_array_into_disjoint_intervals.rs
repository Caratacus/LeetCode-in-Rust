// Tests for Problem 0915: Partition Array into Disjoint Intervals
// Java reference: src/test/java/g0901_1000/s0915_partition_array_into_disjoint_intervals/SolutionTest.java

use leetcode_in_rust::s0915::partition_array_into_disjoint_intervals::Solution;

#[test]
fn test_partition_disjoint() {
    assert_eq!(Solution::partition_disjoint(vec![5, 0, 3, 8, 6]), 3);
}

#[test]
fn test_partition_disjoint2() {
    assert_eq!(Solution::partition_disjoint(vec![1, 1, 1, 0, 6, 12]), 4);
}
