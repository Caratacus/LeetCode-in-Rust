// Tests for Problem 2099: Find Subsequence of Length K With the Largest Sum
// Java reference: src/test/java/g2001_2100/s2099_find_subsequence_of_length_k_with_the_largest_sum/SolutionTest.java

use leetcode_in_rust::s2099::find_subsequence_of_length_k_with_the_largest_sum::Solution;

#[test]
fn test_max_subsequence() {
    assert_eq!(Solution::max_subsequence(vec![2, 1, 3, 3], 2), vec![3, 3]);
}

#[test]
fn test_max_subsequence2() {
    assert_eq!(Solution::max_subsequence(vec![-1, -2, 3, 4], 3), vec![-1, 3, 4]);
}

#[test]
fn test_max_subsequence3() {
    assert_eq!(Solution::max_subsequence(vec![3, 4, 3, 3], 2), vec![4, 3]);
}
