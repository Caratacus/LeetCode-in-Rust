// Tests for Problem 3519: Count Numbers with Non Decreasing Digits
// Java reference: src/test/java/g3501_3600/s3519_count_numbers_with_non_decreasing_digits/SolutionTest.java

use leetcode_in_rust::s3519::count_numbers_with_non_decreasing_digits::Solution;

#[test]
fn test_count_numbers() {
    assert_eq!(Solution::count_numbers("23".to_string(), "28".to_string(), 8), 3);
}

#[test]
fn test_count_numbers2() {
    assert_eq!(Solution::count_numbers("2".to_string(), "7".to_string(), 2), 2);
}
