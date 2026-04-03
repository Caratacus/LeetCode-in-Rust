// Tests for Problem 1799: Maximize Score After N Operations
// Java reference: src/test/java/g1701_1800/s1799_maximize_score_after_n_operations/SolutionTest.java

use leetcode_in_rust::s1799::maximize_score_after_n_operations::Solution;

#[test]
fn test_max_score() {
    assert_eq!(Solution::max_score(vec![1, 2]), 1);
}

#[test]
fn test_max_score2() {
    assert_eq!(Solution::max_score(vec![3, 4, 6, 8]), 11);
}

#[test]
fn test_max_score3() {
    assert_eq!(Solution::max_score(vec![1, 2, 3, 4, 5, 6]), 14);
}
