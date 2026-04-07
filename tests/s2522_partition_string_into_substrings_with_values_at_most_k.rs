// Tests for Problem 2522: Partition String Into Substrings With Values at Most K
// Java reference: src/test/java/g2401_2500/s2522_partition_string_into_substrings_with_values_at_most_k/SolutionTest.java

use leetcode_in_rust::s2522::partition_string_into_substrings_with_values_at_most_k::Solution;

#[test]
fn test_minimum_partition() {
    assert_eq!(Solution::minimum_partition("165462".to_string(), 60), 4);
}

#[test]
fn test_minimum_partition2() {
    assert_eq!(Solution::minimum_partition("238182".to_string(), 5), -1);
}
