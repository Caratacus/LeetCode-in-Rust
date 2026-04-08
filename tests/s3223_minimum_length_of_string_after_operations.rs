// Tests for Problem 3223: Minimum Length of String After Operations
// Java reference: src/test/java/g3201_3300/s3223_minimum_length_of_string_after_operations/SolutionTest.java

use leetcode_in_rust::s3223::minimum_length_of_string_after_operations::Solution;

#[test]
fn test_minimum_length() {
    assert_eq!(Solution::minimum_length("abaacbcbb".to_string()), 5);
}

#[test]
fn test_minimum_length2() {
    assert_eq!(Solution::minimum_length("aa".to_string()), 2);
}
