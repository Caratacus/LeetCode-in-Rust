// Tests for Problem 3399: Smallest Substring With Identical Characters II
// Java reference: src/test/java/g3301_3400/s3399_smallest_substring_with_identical_characters_ii/SolutionTest.java

use leetcode_in_rust::s3399::smallest_substring_with_identical_characters_ii::Solution;

#[test]
fn test_min_length() {
    assert_eq!(Solution::min_length("000001".to_string(), 1), 2);
}

#[test]
fn test_min_length2() {
    assert_eq!(Solution::min_length("0000".to_string(), 2), 1);
}

#[test]
fn test_min_length3() {
    assert_eq!(Solution::min_length("0101".to_string(), 0), 1);
}

#[test]
fn test_min_length4() {
    assert_eq!(Solution::min_length("000".to_string(), 0), 3);
}

#[test]
fn test_min_length5() {
    assert_eq!(Solution::min_length("000001".to_string(), 1), 2);
}
