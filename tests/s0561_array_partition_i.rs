// Tests for Problem 0561: Array Partition I
// Java reference: src/test/java/g0501_0600/s0561_array_partition_i/SolutionTest.java

use leetcode_in_rust::s0561::array_partition_i::Solution;

#[test]
fn test_array_pair_sum() {
    assert_eq!(Solution::array_pair_sum(vec![1, 4, 3, 2]), 4);
}

#[test]
fn test_array_pair_sum2() {
    assert_eq!(Solution::array_pair_sum(vec![6, 2, 6, 5, 1, 2]), 9);
}
