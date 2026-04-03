// Tests for Problem 1663: Smallest String With a Given Numeric Value
// Java reference: src/test/java/g1601_1700/s1663_smallest_string_with_a_given_numeric_value/SolutionTest.java

use leetcode_in_rust::s1663::smallest_string_with_a_given_numeric_value::Solution;

#[test]
fn test_get_smallest_string() {
    assert_eq!(Solution::get_smallest_string(3, 27), "aay".to_string());
}

#[test]
fn test_get_smallest_string2() {
    assert_eq!(Solution::get_smallest_string(5, 73), "aaszz".to_string());
}
