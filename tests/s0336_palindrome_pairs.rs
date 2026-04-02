// Tests for Problem 0336: Palindrome Pairs
// Java reference: src/test/java/g0301_0400/s0336_palindrome_pairs/SolutionTest.java

use leetcode_in_rust::s0336::palindrome_pairs::Solution;

#[test]
fn test_palindrome_pairs() {
    let mut result = Solution::palindrome_pairs(vec!["abcd".to_string(), "dcba".to_string(), "lls".to_string(), "s".to_string(), "sssll".to_string()]);
    result.sort();
    assert!(result.contains(&vec![0, 1]));
    assert!(result.contains(&vec![1, 0]));
}

#[test]
fn test_palindrome_pairs2() {
    let result = Solution::palindrome_pairs(vec!["bat".to_string(), "tab".to_string(), "cat".to_string()]);
    assert!(result.contains(&vec![0, 1]) || result.contains(&vec![1, 0]));
}
