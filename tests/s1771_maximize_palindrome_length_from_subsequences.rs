// Tests for Problem 1771: Maximize Palindrome Length From Subsequences
// Java reference: src/test/java/g1701_1800/s1771_maximize_palindrome_length_from_subsequences/SolutionTest.java

use leetcode_in_rust::s1771::maximize_palindrome_length_from_subsequences::Solution;

#[test]
fn test_longest_palindrome() {
    assert_eq!(
        Solution::longest_palindrome("cacb".to_string(), "cbba".to_string()),
        5
    );
}

#[test]
fn test_longest_palindrome2() {
    assert_eq!(
        Solution::longest_palindrome("ab".to_string(), "ab".to_string()),
        3
    );
}

#[test]
fn test_longest_palindrome3() {
    assert_eq!(
        Solution::longest_palindrome("aa".to_string(), "bb".to_string()),
        0
    );
}
