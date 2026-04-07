// Tests for Problem 2384: Largest Palindromic Number
// Java reference: src/test/java/g2301_2400/s2384_largest_palindromic_number/SolutionTest.java

use leetcode_in_rust::s2384::largest_palindromic_number::Solution;

#[test]
fn test_largest_palindromic() {
    assert_eq!(
        Solution::largest_palindromic("444947137".to_string()),
        "7449447"
    );
}

#[test]
fn test_largest_palindromic2() {
    assert_eq!(Solution::largest_palindromic("00009".to_string()), "9");
}
