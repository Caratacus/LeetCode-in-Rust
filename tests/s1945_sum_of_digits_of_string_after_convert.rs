// Tests for Problem 1945: Sum of Digits of String After Convert
// Java reference: src/test/java/g1901_2000/s1945_sum_of_digits_of_string_after_convert/SolutionTest.java

use leetcode_in_rust::s1945::sum_of_digits_of_string_after_convert::Solution;

#[test]
fn test_get_lucky() {
    assert_eq!(Solution::get_lucky(String::from("iiii"), 1), 36);
}

#[test]
fn test_get_lucky2() {
    assert_eq!(Solution::get_lucky(String::from("leetcode"), 2), 6);
}

#[test]
fn test_get_lucky3() {
    assert_eq!(Solution::get_lucky(String::from("zbax"), 2), 8);
}
