// Tests for Problem 1793: Maximum Score of a Good Subarray
// Java reference: src/test/java/g1701_1800/s1793_maximum_score_of_a_good_subarray/SolutionTest.java

use leetcode_in_rust::s1793::maximum_score_of_a_good_subarray::Solution;

#[test]
fn test_maximum_score() {
    assert_eq!(Solution::maximum_score(vec![1, 4, 3, 7, 4, 5], 3), 15);
}

#[test]
fn test_maximum_score2() {
    assert_eq!(Solution::maximum_score(vec![5, 5, 4, 5, 4, 1, 1, 1], 0), 20);
}
