// Tests for Problem 0028: Find the Index of the First Occurrence in a String
// Java reference: src/test/java/g0001_0100/s0028_find_the_index_of_the_first_occurrence_in_a_string/SolutionTest.java

use leetcode_in_rust::s0028::find_the_index_of_the_first_occurrence_in_a_string::Solution;

#[test]
fn test_str_str() {
    assert_eq!(Solution::str_str("hello".to_string(), "ll".to_string()), 2);
}

#[test]
fn test_str_str2() {
    assert_eq!(Solution::str_str("hello".to_string(), "".to_string()), 0);
}

#[test]
fn test_str_str3() {
    assert_eq!(Solution::str_str("hello".to_string(), "oo".to_string()), -1);
}

#[test]
fn test_str_str4() {
    assert_eq!(Solution::str_str("".to_string(), "".to_string()), 0);
}
