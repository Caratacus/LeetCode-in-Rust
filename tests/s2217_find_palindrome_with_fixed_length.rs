// Tests for Problem 2217: Find Palindrome With Fixed Length
// Java reference: src/test/java/g2201_2300/s2217_find_palindrome_with_fixed_length/SolutionTest.java

use leetcode_in_rust::s2217::find_palindrome_with_fixed_length::Solution;

#[test]
fn test_kth_palindrome() {
    assert_eq!(
        Solution::kth_palindrome(vec![1, 2, 3, 4, 5, 90], 3),
        vec![101, 111, 121, 131, 141, 999]
    );
}

#[test]
fn test_kth_palindrome2() {
    assert_eq!(
        Solution::kth_palindrome(vec![2, 4, 6], 4),
        vec![1111, 1331, 1551]
    );
}
