// Tests for Problem 1278: Palindrome Partitioning III
// Java reference: src/test/java/g1201_1300/s1278_palindrome_partitioning_iii/SolutionTest.java

use leetcode_in_rust::s1278::palindrome_partitioning_iii::Solution;

#[test]
fn test_palindrome_partition() {
    let result = Solution::palindrome_partition("abc".to_string(), 2);
    assert_eq!(result, 1);
}

#[test]
fn test_palindrome_partition2() {
    let result = Solution::palindrome_partition("aabbc".to_string(), 3);
    assert_eq!(result, 0);
}

#[test]
fn test_palindrome_partition3() {
    let result = Solution::palindrome_partition("leetcode".to_string(), 8);
    assert_eq!(result, 0);
}
