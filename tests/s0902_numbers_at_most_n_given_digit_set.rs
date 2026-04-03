// Tests for Problem 0902: Numbers At Most N Given Digit Set
// Java reference: src/test/java/g0901_1000/s0902_numbers_at_most_n_given_digit_set/SolutionTest.java

use leetcode_in_rust::s0902::numbers_at_most_n_given_digit_set::Solution;

#[test]
fn test_at_most_n_given_digit_set() {
    let result = Solution::at_most_n_given_digit_set(vec!["1".to_string(), "3".to_string(), "5".to_string(), "7".to_string()], 100);
    assert_eq!(result, 20);
}

#[test]
fn test_at_most_n_given_digit_set2() {
    let result = Solution::at_most_n_given_digit_set(vec!["1".to_string(), "4".to_string(), "9".to_string()], 1000000000);
    assert_eq!(result, 29523);
}

#[test]
fn test_at_most_n_given_digit_set3() {
    let result = Solution::at_most_n_given_digit_set(vec!["7".to_string()], 8);
    assert_eq!(result, 1);
}
