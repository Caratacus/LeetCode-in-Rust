// Tests for Problem 3165: Maximum Sum of Subsequence With Non-Adjacent Elements
// Java reference: src/test/java/g3101_3200/s3165_maximum_sum_of_subsequence_with_non_adjacent_elements/SolutionTest.java

use leetcode_in_rust::s3165::maximum_sum_of_subsequence_with_non_adjacent_elements::Solution;
#[test]
fn test_maximum_sum_subsequence() {
    assert_eq!(
        Solution::maximum_sum_subsequence(vec![3, 5, 9], vec![vec![1, -2], vec![0, -3]]),
        21
    );
}
#[test]
fn test_maximum_sum_subsequence2() {
    assert_eq!(Solution::maximum_sum_subsequence(vec![0, -1], vec![vec![0, -5]]), 0);
}
