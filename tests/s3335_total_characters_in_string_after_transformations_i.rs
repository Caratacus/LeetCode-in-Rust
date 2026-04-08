// Tests for Problem 3335: Total Characters in String After Transformations I
// Java reference: src/test/java/g3301_3400/s3335_total_characters_in_string_after_transformations_i/SolutionTest.java

use leetcode_in_rust::s3335::total_characters_in_string_after_transformations_i::Solution;

#[test]
fn test_length_after_transformations() {
    assert_eq!(Solution::length_after_transformations("abcyy".to_string(), 2), 7);
}

#[test]
fn test_length_after_transformations2() {
    assert_eq!(Solution::length_after_transformations("azbk".to_string(), 1), 5);
}
