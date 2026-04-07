// Tests for Problem 2108: Find First Palindromic String in the Array
// Java reference: src/test/java/g2101_2200/s2108_find_first_palindromic_string_in_the_array/SolutionTest.java

use leetcode_in_rust::s2108::find_first_palindromic_string_in_the_array::Solution;

#[test]
fn test_first_palindrome() {
    assert_eq!(
        Solution::first_palindrome(vec![
            "abc".to_string(),
            "car".to_string(),
            "ada".to_string(),
            "racecar".to_string(),
            "cool".to_string()
        ]),
        "ada"
    );
}

#[test]
fn test_first_palindrome2() {
    assert_eq!(
        Solution::first_palindrome(vec!["notapalindrome".to_string(), "racecar".to_string()]),
        "racecar"
    );
}

#[test]
fn test_first_palindrome3() {
    assert_eq!(
        Solution::first_palindrome(vec!["def".to_string(), "ghi".to_string()]),
        ""
    );
}
