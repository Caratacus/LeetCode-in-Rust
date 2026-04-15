// Tests for Problem 3702: Longest Subsequence with Non Zero Bitwise XOR
// Java reference: src/test/java/g3701_3800/s3702_longest_subsequence_with_non_zero_bitwise_xor/SolutionTest.java
use leetcode_in_rust::s3702::longest_subsequence_with_non_zero_bitwise_xor::Solution;
#[test]
fn test_longest_subsequence() { assert_eq!(Solution::longest_subsequence(vec![1, 2, 3]), 2); }
#[test]
fn test_longest_subsequence2() { assert_eq!(Solution::longest_subsequence(vec![2, 3, 4]), 3); }
