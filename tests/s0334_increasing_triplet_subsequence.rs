// Tests for Problem 0334: Increasing Triplet Subsequence
// Java reference: src/test/java/g0301_0400/s0334_increasing_triplet_subsequence/SolutionTest.java

use leetcode_in_rust::s0334::increasing_triplet_subsequence::Solution;

#[test]
fn test_increasing_triplet() {
    assert_eq!(Solution::increasing_triplet(vec![1, 2, 3, 4, 5]), true);
}

#[test]
fn test_increasing_triplet2() {
    assert_eq!(Solution::increasing_triplet(vec![5, 4, 3, 2, 1]), false);
}

#[test]
fn test_increasing_triplet3() {
    assert_eq!(Solution::increasing_triplet(vec![2, 1, 5, 0, 4, 6]), true);
}
