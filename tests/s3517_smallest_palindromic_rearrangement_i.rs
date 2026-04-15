// Tests for Problem 3517: Smallest Palindromic Rearrangement I
// Java reference: src/test/java/g3501_3600/s3517_smallest_palindromic_rearrangement_i/SolutionTest.java

use leetcode_in_rust::s3517::smallest_palindromic_rearrangement_i::Solution;

#[test]
fn test_smallest_palindrome() {
    assert_eq!(Solution::smallest_palindrome("z".to_string()), "z".to_string());
}

#[test]
fn test_smallest_palindrome2() {
    assert_eq!(Solution::smallest_palindrome("babab".to_string()), "abbba".to_string());
}

#[test]
fn test_smallest_palindrome3() {
    assert_eq!(Solution::smallest_palindrome("daccad".to_string()), "acddca".to_string());
}
