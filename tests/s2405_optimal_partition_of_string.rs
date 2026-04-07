// Tests for Problem 2405: Optimal Partition of String
// Java reference: src/test/java/g2401_2500/s2405_optimal_partition_of_string/SolutionTest.java

use leetcode_in_rust::s2405::optimal_partition_of_string::Solution;

#[test]
fn test_partition_string() {
    assert_eq!(Solution::partition_string("abacaba".to_string()), 4);
}

#[test]
fn test_partition_string2() {
    assert_eq!(Solution::partition_string("ssssss".to_string()), 6);
}
