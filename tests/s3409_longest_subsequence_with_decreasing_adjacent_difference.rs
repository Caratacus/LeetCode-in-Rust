// Tests for Problem 3409: Longest Subsequence with Decreasing Adjacent Difference
// Java reference: src/test/java/g3401_3500/s3409_longest_subsequence_with_decreasing_adjacent_difference/SolutionTest.java

use leetcode_in_rust::s3409::longest_subsequence_with_decreasing_adjacent_difference::Solution;

#[test]
fn test_longest_subsequence() {
    assert_eq!(Solution::longest_subsequence(vec![16, 6, 3]), 3);
}

#[test]
fn test_longest_subsequence2() {
    assert_eq!(Solution::longest_subsequence(vec![6, 5, 3, 4, 2, 1]), 4);
}

#[test]
fn test_longest_subsequence3() {
    assert_eq!(Solution::longest_subsequence(vec![10, 20, 10, 19, 10, 20]), 5);
}
