// Tests for Problem 2389: Longest Subsequence With Limited Sum
// Java reference: src/test/java/g2301_2400/s2389_longest_subsequence_with_limited_sum/SolutionTest.java

use leetcode_in_rust::s2389::longest_subsequence_with_limited_sum::Solution;

#[test]
fn test_answer_queries() {
    assert_eq!(
        Solution::answer_queries(vec![4, 5, 2, 1], vec![3, 10, 21]),
        vec![2, 3, 4]
    );
}

#[test]
fn test_answer_queries2() {
    assert_eq!(Solution::answer_queries(vec![2, 3, 4, 5], vec![1]), vec![0]);
}
