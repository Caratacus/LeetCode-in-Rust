// Tests for Problem 3174: Clear Digits
// Java reference: src/test/java/g3101_3200/s3174_clear_digits/SolutionTest.java

use leetcode_in_rust::s3174::clear_digits::Solution;

#[test]
fn test_clear_digits() {
    assert_eq!(Solution::clear_digits(String::from("abc")), String::from("abc"));
}
#[test]
fn test_clear_digits2() {
    assert_eq!(Solution::clear_digits(String::from("cb34")), String::from(""));
}