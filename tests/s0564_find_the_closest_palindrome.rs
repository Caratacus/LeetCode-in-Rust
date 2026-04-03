// Tests for Problem 0564: Find the Closest Palindrome
// Java reference: src/test/java/g0501_0600/s0564_find_the_closest_palindrome/SolutionTest.java

use leetcode_in_rust::s0564::find_the_closest_palindrome::Solution;

#[test]
fn test_nearest_palindromic() {
    assert_eq!(Solution::nearest_palindromic("123".to_string()), "121");
}

#[test]
fn test_nearest_palindromic2() {
    assert_eq!(Solution::nearest_palindromic("1".to_string()), "0");
}

#[test]
fn test_nearest_palindromic3() {
    assert_eq!(Solution::nearest_palindromic("121".to_string()), "111");
}

#[test]
fn test_nearest_palindromic4() {
    assert_eq!(Solution::nearest_palindromic("100".to_string()), "99");
}

#[test]
fn test_nearest_palindromic5() {
    assert_eq!(Solution::nearest_palindromic("2".to_string()), "1");
}

#[test]
fn test_nearest_palindromic6() {
    assert_eq!(Solution::nearest_palindromic("999".to_string()), "1001");
}

#[test]
fn test_nearest_palindromic7() {
    assert_eq!(Solution::nearest_palindromic("1221".to_string()), "1111");
}

#[test]
fn test_nearest_palindromic8() {
    assert_eq!(Solution::nearest_palindromic("12321".to_string()), "12221");
}

#[test]
fn test_nearest_palindromic9() {
    assert_eq!(Solution::nearest_palindromic("1000001".to_string()), "999999");
}

#[test]
fn test_nearest_palindromic10() {
    assert_eq!(Solution::nearest_palindromic("10".to_string()), "9");
}

#[test]
fn test_nearest_palindromic11() {
    assert_eq!(Solution::nearest_palindromic("11".to_string()), "9");
}

#[test]
fn test_nearest_palindromic12() {
    assert_eq!(Solution::nearest_palindromic("807".to_string()), "808");
}

#[test]
fn test_nearest_palindromic13() {
    assert_eq!(Solution::nearest_palindromic("1000".to_string()), "999");
}
