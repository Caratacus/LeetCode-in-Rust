// Tests for Problem 2767: Partition String Into Minimum Beautiful Substrings
// Java reference: src/test/java/g2701_2800/s2767_partition_string_into_minimum_beautiful_substrings/SolutionTest.java

use leetcode_in_rust::s2767::partition_string_into_minimum_beautiful_substrings::Solution;

#[test]
fn test_minimum_beautiful_substrings() {
    assert_eq!(Solution::minimum_beautiful_substrings("1011".to_string()), 2);
}

#[test]
fn test_minimum_beautiful_substrings2() {
    assert_eq!(Solution::minimum_beautiful_substrings("0".to_string()), -1);
}
