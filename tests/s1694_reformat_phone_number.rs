// Tests for Problem 1694: Reformat Phone Number
// Java reference: src/test/java/g1601_1700/s1694_reformat_phone_number/SolutionTest.java

use leetcode_in_rust::s1694::reformat_phone_number::Solution;

#[test]
fn test_reformat_number() {
    assert_eq!(Solution::reformat_number("1-23-45 6".to_string()), "123-456".to_string());
}

#[test]
fn test_reformat_number2() {
    assert_eq!(Solution::reformat_number("123 4-567".to_string()), "123-45-67".to_string());
}

#[test]
fn test_reformat_number3() {
    assert_eq!(Solution::reformat_number("123 4-5678".to_string()), "123-456-78".to_string());
}
