// Tests for Problem 1689: Partitioning into Minimum Number of Deci-Binary Numbers
// Java reference: src/test/java/g1601_1700/s1689_partitioning_into_minimum_number_of_deci_binary_numbers/SolutionTest.java

use leetcode_in_rust::s1689::partitioning_into_minimum_number_of_deci_binary_numbers::Solution;

#[test]
fn test_min_partitions() {
    assert_eq!(Solution::min_partitions("32".to_string()), 3);
}

#[test]
fn test_min_partitions2() {
    assert_eq!(Solution::min_partitions("82734".to_string()), 8);
}

#[test]
fn test_min_partitions3() {
    assert_eq!(Solution::min_partitions("27346209830709182346".to_string()), 9);
}
