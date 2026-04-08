// Tests for Problem 3170: Lexicographically Minimum String after Removing Stars
// Java reference: src/test/java/g3101_3200/s3170_lexicographically_minimum_string_after_removing_stars/SolutionTest.java

use leetcode_in_rust::s3170::lexicographically_minimum_string_after_removing_stars::Solution;

#[test]
fn test_clear_stars() {
    assert_eq!(Solution::clear_stars(String::from("aaba*")), String::from("aab"));
}
#[test]
fn test_clear_stars2() {
    assert_eq!(Solution::clear_stars(String::from("abc")), String::from("abc"));
}