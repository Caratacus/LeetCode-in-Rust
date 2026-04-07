// Tests for Problem 2311: Longest Binary Subsequence Less Than or Equal to K
// Java reference: src/test/java/g2301_2400/s2311_longest_binary_subsequence_less_than_or_equal_to_k/SolutionTest.java

use leetcode_in_rust::s2311::longest_binary_subsequence_less_than_or_equal_to_k::Solution;

#[test]
fn test_longest_subsequence() {
    assert_eq!(Solution::longest_subsequence(String::from("1001010"), 5), 5);
}

#[test]
fn test_longest_subsequence2() {
    assert_eq!(Solution::longest_subsequence(String::from("00101001"), 1), 6);
}
