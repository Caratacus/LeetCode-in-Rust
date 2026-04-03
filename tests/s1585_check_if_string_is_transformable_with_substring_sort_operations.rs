// Tests for Problem 1585: Check If String Is Transformable With Substring Sort Operations
// Java reference: src/test/java/g1501_1600/s1585_check_if_string_is_transformable_with_substring_sort_operations/SolutionTest.java

use leetcode_in_rust::s1585::check_if_string_is_transformable_with_substring_sort_operations::Solution;

#[test]
fn test_is_transformable() {
    assert!(Solution::is_transformable("84532".to_string(), "34852".to_string()));
}

#[test]
fn test_is_transformable2() {
    assert!(Solution::is_transformable("34521".to_string(), "23415".to_string()));
}

#[test]
fn test_is_transformable3() {
    assert!(!Solution::is_transformable("12345".to_string(), "12435".to_string()));
}
