// Tests for Problem 0357: Count Numbers with Unique Digits
// Java reference: src/test/java/g0301_0400/s0357_count_numbers_with_unique_digits/SolutionTest.java

use leetcode_in_rust::s0357::count_numbers_with_unique_digits::Solution;

#[test]
fn test_count_numbers_with_unique_digits() {
    assert_eq!(Solution::count_numbers_with_unique_digits(2), 91);
}

#[test]
fn test_count_numbers_with_unique_digits2() {
    assert_eq!(Solution::count_numbers_with_unique_digits(0), 1);
}
