// Tests for Problem 3260: Find the Largest Palindrome Divisible by K
// Java reference: src/test/java/g3201_3300/s3260_find_the_largest_palindrome_divisible_by_k/SolutionTest.java

use leetcode_in_rust::s3260::find_the_largest_palindrome_divisible_by_k::Solution;

#[test]
fn test_largest_palindrome() {
    assert_eq!(Solution::largest_palindrome(3, 5), "595");
}

#[test]
fn test_largest_palindrome2() {
    assert_eq!(Solution::largest_palindrome(1, 4), "8");
}

#[test]
fn test_largest_palindrome3() {
    assert_eq!(Solution::largest_palindrome(5, 6), "89898");
}

#[test]
fn test_largest_palindrome4() {
    assert_eq!(Solution::largest_palindrome(1, 1), "9");
    assert_eq!(Solution::largest_palindrome(2, 1), "99");
    assert_eq!(Solution::largest_palindrome(3, 1), "999");
}

#[test]
fn test_largest_palindrome5() {
    assert_eq!(Solution::largest_palindrome(1, 2), "8");
    assert_eq!(Solution::largest_palindrome(2, 2), "88");
    assert_eq!(Solution::largest_palindrome(3, 2), "898");
    assert_eq!(Solution::largest_palindrome(4, 2), "8998");
}

#[test]
fn test_largest_palindrome6() {
    assert_eq!(Solution::largest_palindrome(1, 3), "9");
    assert_eq!(Solution::largest_palindrome(2, 3), "99");
    assert_eq!(Solution::largest_palindrome(3, 3), "999");
}

#[test]
fn test_largest_palindrome7() {
    assert_eq!(Solution::largest_palindrome(1, 4), "8");
    assert_eq!(Solution::largest_palindrome(2, 4), "88");
    assert_eq!(Solution::largest_palindrome(3, 4), "888");
    assert_eq!(Solution::largest_palindrome(4, 4), "8888");
    assert_eq!(Solution::largest_palindrome(5, 4), "88988");
}

#[test]
fn test_largest_palindrome8() {
    assert_eq!(Solution::largest_palindrome(1, 5), "5");
    assert_eq!(Solution::largest_palindrome(2, 5), "55");
    assert_eq!(Solution::largest_palindrome(3, 5), "595");
}

#[test]
fn test_largest_palindrome9() {
    assert_eq!(Solution::largest_palindrome(1, 6), "6");
    assert_eq!(Solution::largest_palindrome(2, 6), "66");
    assert_eq!(Solution::largest_palindrome(4, 6), "8778");
}

#[test]
fn test_largest_palindrome10() {
    assert_eq!(Solution::largest_palindrome(1, 7), "7");
    assert_eq!(Solution::largest_palindrome(2, 7), "77");
    assert_eq!(Solution::largest_palindrome(3, 7), "959");
    assert_eq!(Solution::largest_palindrome(5, 7), "99799");
    assert_eq!(Solution::largest_palindrome(6, 7), "999999");
    assert_eq!(Solution::largest_palindrome(7, 7), "9994999");
}

#[test]
fn test_largest_palindrome11() {
    assert_eq!(Solution::largest_palindrome(1, 8), "8");
    assert_eq!(Solution::largest_palindrome(2, 8), "88");
    assert_eq!(Solution::largest_palindrome(3, 8), "888");
    assert_eq!(Solution::largest_palindrome(4, 8), "8888");
    assert_eq!(Solution::largest_palindrome(5, 8), "88888");
}

#[test]
fn test_largest_palindrome12() {
    assert_eq!(Solution::largest_palindrome(1, 9), "9");
    assert_eq!(Solution::largest_palindrome(2, 9), "99");
    assert_eq!(Solution::largest_palindrome(3, 9), "999");
}
