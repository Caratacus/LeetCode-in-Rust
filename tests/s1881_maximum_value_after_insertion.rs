// Tests for Problem 1881: Maximum Value after Insertion
// Java reference: src/test/java/g1801_1900/s1881_maximum_value_after_insertion/SolutionTest.java

use leetcode_in_rust::s1881::maximum_value_after_insertion::Solution;

#[test]
fn test_max_value() {
    assert_eq!(Solution::max_value("99".to_string(), 9), "999".to_string());
}

#[test]
fn test_max_value2() {
    assert_eq!(Solution::max_value("-13".to_string(), 2), "-123".to_string());
}
