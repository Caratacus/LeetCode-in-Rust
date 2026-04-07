// Tests for Problem 2396: Strictly Palindromic Number
// Java reference: src/test/java/g2301_2400/s2396_strictly_palindromic_number/SolutionTest.java

use leetcode_in_rust::s2396::strictly_palindromic_number::Solution;

#[test]
fn test_is_strictly_palindromic() {
    assert_eq!(Solution::is_strictly_palindromic(9), false);
}

#[test]
fn test_is_strictly_palindromic2() {
    assert_eq!(Solution::is_strictly_palindromic(4), false);
}

#[test]
fn test_is_strictly_palindromic3() {
    assert_eq!(Solution::is_strictly_palindromic(9779), false);
}

#[test]
fn test_is_strictly_palindromic4() {
    assert_eq!(Solution::is_strictly_palindromic(3), true);
}

#[test]
fn test_is_strictly_palindromic5() {
    assert_eq!(Solution::is_strictly_palindromic(2), true);
}

#[test]
fn test_is_strictly_palindromic6() {
    assert_eq!(Solution::is_strictly_palindromic(1), true);
}

#[test]
fn test_is_strictly_palindromic7() {
    assert_eq!(Solution::is_strictly_palindromic(10000), false);
}
