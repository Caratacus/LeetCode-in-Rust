// Tests for Problem 2645: Minimum Additions to Make Valid String
// Java reference: src/test/java/g2601_2700/s2645_minimum_additions_to_make_valid_string/SolutionTest.java

use leetcode_in_rust::s2645::minimum_additions_to_make_valid_string::Solution;

#[test]
fn test_add_minimum() {
    assert_eq!(Solution::add_minimum("b".to_string()), 2);
}

#[test]
fn test_add_minimum2() {
    assert_eq!(Solution::add_minimum("aaa".to_string()), 6);
}

#[test]
fn test_add_minimum3() {
    assert_eq!(Solution::add_minimum("abc".to_string()), 0);
}
