// Tests for Problem 0763: Partition Labels
// Java reference: src/test/java/g0701_0800/s0763_partition_labels/SolutionTest.java

use leetcode_in_rust::s0763::partition_labels::Solution;

#[test]
fn test_partition_labels() {
    assert_eq!(Solution::partition_labels("ababcbacadefegdehijhklij".to_string()), vec![9, 7, 8]);
}

#[test]
fn test_partition_labels2() {
    assert_eq!(Solution::partition_labels("eccbbbbdec".to_string()), vec![10]);
}
