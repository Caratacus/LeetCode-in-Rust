// Tests for Problem 0001: Two Sum
// Java reference: src/test/java/g0001_0100/s0001_two_sum/SolutionTest.java

use leetcode_in_rust::s0001::two_sum::Solution;

#[test]
fn test_two_sum() {
    assert_eq!(Solution::two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
}

#[test]
fn test_two_sum2() {
    assert_eq!(Solution::two_sum(vec![3, 2, 4], 6), vec![1, 2]);
}

#[test]
fn test_two_sum3() {
    assert_eq!(Solution::two_sum(vec![3, 3], 6), vec![0, 1]);
}

#[test]
fn test_two_sum4() {
    assert_eq!(Solution::two_sum(vec![3, 3], 7), vec![-1, -1]);
}
