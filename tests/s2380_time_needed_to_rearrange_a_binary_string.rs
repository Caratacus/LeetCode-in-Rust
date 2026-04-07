// Tests for Problem 2380: Time Needed to Rearrange a Binary String
// Java reference: src/test/java/g2301_2400/s2380_time_needed_to_rearrange_a_binary_string/SolutionTest.java

use leetcode_in_rust::s2380::time_needed_to_rearrange_a_binary_string::Solution;

#[test]
fn test_seconds_to_remove_occurrences() {
    assert_eq!(Solution::seconds_to_remove_occurrences("0110101".to_string()), 4);
}

#[test]
fn test_seconds_to_remove_occurrences2() {
    assert_eq!(Solution::seconds_to_remove_occurrences("11100".to_string()), 0);
}
