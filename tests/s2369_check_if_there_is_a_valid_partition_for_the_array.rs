// Tests for Problem 2369: Check if There is a Valid Partition For the Array
// Java reference: src/test/java/g2301_2400/s2369_check_if_there_is_a_valid_partition_for_the_array/SolutionTest.java

use leetcode_in_rust::s2369::check_if_there_is_a_valid_partition_for_the_array::Solution;

#[test]
fn test_valid_partition() {
    assert_eq!(Solution::valid_partition(vec![4, 4, 4, 5, 6]), true);
}

#[test]
fn test_valid_partition2() {
    assert_eq!(Solution::valid_partition(vec![1, 1, 1, 2]), false);
}

#[test]
fn test_valid_partition3() {
    assert_eq!(Solution::valid_partition(vec![1, 2]), false);
}

#[test]
fn test_valid_partition4() {
    assert_eq!(Solution::valid_partition(vec![1, 3]), false);
}
