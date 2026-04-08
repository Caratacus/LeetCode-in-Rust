// Tests for Problem 3035: Maximum Palindromes After Operations
// Java reference: src/test/java/g3001_3100/s3035_maximum_palindromes_after_operations/SolutionTest.java

use leetcode_in_rust::s3035::maximum_palindromes_after_operations::Solution;

#[test]
fn test_max_palindromes_after_operations() {
    let words = vec![
        String::from("abbb"),
        String::from("ba"),
        String::from("aa"),
    ];
    assert_eq!(Solution::max_palindromes_after_operations(words), 3);
}

#[test]
fn test_max_palindromes_after_operations2() {
    let words = vec![String::from("abc"), String::from("ab")];
    assert_eq!(Solution::max_palindromes_after_operations(words), 2);
}

#[test]
fn test_max_palindromes_after_operations3() {
    let words = vec![String::from("cd"), String::from("ef"), String::from("a")];
    assert_eq!(Solution::max_palindromes_after_operations(words), 1);
}
