// Tests for Problem 2911: Minimum Changes to Make K Semi-palindromes
// Java reference: src/test/java/g2901_3000/s2911_minimum_changes_to_make_k_semi_palindromes/SolutionTest.java

use leetcode_in_rust::s2911::minimum_changes_to_make_k_semi_palindromes::Solution;

#[test]
fn test_minimum_changes() {
    assert_eq!(Solution::minimum_changes("abcac".to_string(), 2), 1);
}

#[test]
fn test_minimum_changes2() {
    assert_eq!(Solution::minimum_changes("abcdef".to_string(), 2), 2);
}

#[test]
fn test_minimum_changes3() {
    assert_eq!(Solution::minimum_changes("aabbaa".to_string(), 3), 0);
}
