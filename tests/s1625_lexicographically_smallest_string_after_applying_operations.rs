// Tests for Problem 1625: Lexicographically Smallest String After Applying Operations
// Java reference: src/test/java/g1601_1700/s1625_lexicographically_smallest_string_after_applying_operations/SolutionTest.java

use leetcode_in_rust::s1625::lexicographically_smallest_string_after_applying_operations::Solution;

#[test]
fn test_find_lex_smallest_string() {
    assert_eq!(Solution::find_lex_smallest_string("5525".to_string(), 9, 2), "2050");
}

#[test]
fn test_find_lex_smallest_string2() {
    assert_eq!(Solution::find_lex_smallest_string("74".to_string(), 5, 1), "24");
}

#[test]
fn test_find_lex_smallest_string3() {
    assert_eq!(Solution::find_lex_smallest_string("0011".to_string(), 4, 2), "0011");
}
