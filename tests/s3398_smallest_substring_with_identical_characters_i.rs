// Tests for Problem 3398: Smallest Substring With Identical Characters I
// Java reference: src/test/java/g3301_3400/s3398_smallest_substring_with_identical_characters_i/SolutionTest.java

use leetcode_in_rust::s3398::smallest_substring_with_identical_characters_i::Solution;

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
    assert_eq!(Solution::min_length("000".to_string(), 2), 1);
}
