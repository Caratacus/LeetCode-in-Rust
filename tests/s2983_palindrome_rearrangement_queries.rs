// Tests for Problem 2983: Palindrome Rearrangement Queries
// Java reference: src/test/java/g2901_3000/s2983_palindrome_rearrangement_queries/SolutionTest.java

use leetcode_in_rust::s2983::palindrome_rearrangement_queries::Solution;

#[test]
fn test_can_make_palindrome_queries() {
    assert_eq!(
        Solution::can_make_palindrome_queries("abcabc".to_string(), vec![vec![1, 1, 3, 5], vec![0, 2, 5, 5]]),
        vec![true, true]
    );
}

#[test]
fn test_can_make_palindrome_queries2() {
    assert_eq!(
        Solution::can_make_palindrome_queries("abbcdecbba".to_string(), vec![vec![0, 2, 7, 9]]),
        vec![false]
    );
}

#[test]
fn test_can_make_palindrome_queries3() {
    assert_eq!(
        Solution::can_make_palindrome_queries("acbcab".to_string(), vec![vec![1, 2, 4, 5]]),
        vec![true]
    );
}
