// Tests for Problem 1012: Numbers With Repeated Digits
// Java reference: src/test/java/g1001_1100/s1012_numbers_with_repeated_digits/SolutionTest.java

use leetcode_in_rust::s1012::numbers_with_repeated_digits::Solution;

#[test]
fn test_num_dup_digits_at_most_n() {
    assert_eq!(Solution::num_dup_digits_at_most_n(20), 1);
}

#[test]
fn test_num_dup_digits_at_most_n2() {
    assert_eq!(Solution::num_dup_digits_at_most_n(100), 10);
}

#[test]
fn test_num_dup_digits_at_most_n3() {
    assert_eq!(Solution::num_dup_digits_at_most_n(1000), 262);
}
