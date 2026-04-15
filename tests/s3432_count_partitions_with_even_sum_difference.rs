// Tests for Problem 3432: Count Partitions with Even Sum Difference
// Java reference: src/test/java/g3401_3500/s3432_count_partitions_with_even_sum_difference/SolutionTest.java

use leetcode_in_rust::s3432::count_partitions_with_even_sum_difference::Solution;

#[test]
fn test_count_partitions() {
    assert_eq!(Solution::count_partitions(vec![10, 10, 3, 7, 6]), 4);
}

#[test]
fn test_count_partitions2() {
    assert_eq!(Solution::count_partitions(vec![1, 2, 2]), 0);
}

#[test]
fn test_count_partitions3() {
    assert_eq!(Solution::count_partitions(vec![2, 4, 6, 8]), 3);
}
