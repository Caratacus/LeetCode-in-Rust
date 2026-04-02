// Tests for Problem 0873: Length of Longest Fibonacci Subsequence
// Java reference: src/test/java/g0801_0900/s0873_length_of_longest_fibonacci_subsequence/SolutionTest.java

use leetcode_in_rust::s0873::length_of_longest_fibonacci_subsequence::Solution;

#[test]
fn test_len_longest_fib_subseq() {
    assert_eq!(Solution::len_longest_fib_subseq(vec![1, 2, 3, 4, 5, 6, 7, 8]), 5);
}

#[test]
fn test_len_longest_fib_subseq2() {
    assert_eq!(Solution::len_longest_fib_subseq(vec![1, 3, 7, 11, 12, 14, 18]), 3);
}
