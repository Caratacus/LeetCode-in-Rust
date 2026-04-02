// Tests for Problem 0016: 3Sum Closest
// Java reference: src/test/java/g0001_0100/s0016_3sum_closest/SolutionTest.java

use leetcode_in_rust::s0016::p3sum_closest::Solution;

#[test]
fn test_three_sum_closest() {
    assert_eq!(Solution::three_sum_closest(vec![-1, 2, 1, -4], 1), 2);
}

#[test]
fn test_three_sum_closest2() {
    assert_eq!(Solution::three_sum_closest(vec![0, 0, 0], 1), 0);
}

#[test]
fn test_three_sum_closest3() {
    assert_eq!(Solution::three_sum_closest(vec![1, 2, 4, 8, 16, 32, 64, 128], 82), 82);
}

#[test]
fn test_three_sum_closest4() {
    assert_eq!(Solution::three_sum_closest(vec![4, 0, 5, -5, 3, 3, 0, -4, -5], -2), -2);
}
