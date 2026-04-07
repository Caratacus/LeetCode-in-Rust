// Tests for Problem 1849: Splitting a String Into Descending Consecutive Values
// Java reference: src/test/java/g1801_1900/s1849_splitting_a_string_into_descending_consecutive_values/SolutionTest.java

use leetcode_in_rust::s1849::splitting_a_string_into_descending_consecutive_values::Solution;

#[test]
fn test_split_string() {
    assert_eq!(Solution::split_string("1234".to_string()), false);
}

#[test]
fn test_split_string2() {
    assert_eq!(Solution::split_string("050043".to_string()), true);
}

#[test]
fn test_split_string3() {
    assert_eq!(Solution::split_string("9080701".to_string()), false);
}
