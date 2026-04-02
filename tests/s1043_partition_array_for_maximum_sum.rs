// Tests for Problem 1043: Partition Array for Maximum Sum
// Java reference: src/test/java/g1001_1100/s1043_partition_array_for_maximum_sum/SolutionTest.java

use leetcode_in_rust::s1043::partition_array_for_maximum_sum::Solution;

#[test]
fn test_max_sum_after_partitioning() {
    assert_eq!(Solution::max_sum_after_partitioning(vec![1, 15, 7, 9, 2, 5, 10], 3), 84);
}

#[test]
fn test_max_sum_after_partitioning2() {
    assert_eq!(
        Solution::max_sum_after_partitioning(vec![1, 4, 1, 5, 7, 3, 6, 1, 9, 9, 3], 4),
        83
    );
}

#[test]
fn test_max_sum_after_partitioning3() {
    assert_eq!(Solution::max_sum_after_partitioning(vec![1], 1), 1);
}
