// Tests for Problem 2926: Maximum Balanced Subsequence Sum
// Java reference: src/test/java/g2901_3000/s2926_maximum_balanced_subsequence_sum/SolutionTest.java

use leetcode_in_rust::s2926::maximum_balanced_subsequence_sum::Solution;

#[test]
fn test_max_balanced_subsequence_sum() {
    assert_eq!(Solution::max_balanced_subsequence_sum(vec![3, 3, 5, 6]), 14);
}

#[test]
fn test_max_balanced_subsequence_sum2() {
    assert_eq!(Solution::max_balanced_subsequence_sum(vec![5, -1, -3, 8]), 13);
}
