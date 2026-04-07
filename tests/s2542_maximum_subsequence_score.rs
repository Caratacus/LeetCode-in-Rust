// Tests for Problem 2542: Maximum Subsequence Score
// Java reference: src/test/java/g2501_2600/s2542_maximum_subsequence_score/SolutionTest.java
use leetcode_in_rust::s2542::maximum_subsequence_score::Solution;

#[test]
fn test_max_score() {
    assert_eq!(Solution::max_score(vec![1, 3, 3, 2], vec![2, 1, 3, 4], 3), 12);
}
#[test]
fn test_max_score2() {
    assert_eq!(Solution::max_score(vec![4, 2, 3, 1, 1], vec![7, 5, 10, 9, 6], 1), 30);
}
