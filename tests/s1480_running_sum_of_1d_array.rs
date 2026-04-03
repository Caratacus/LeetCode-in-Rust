// Tests for Problem 1480: Running Sum of 1d Array
// Java reference: src/test/java/g1401_1500/s1480_running_sum_of_1d_array/SolutionTest.java

use leetcode_in_rust::s1480::running_sum_of_1d_array::Solution;

#[test]
fn test_running_sum() {
    assert_eq!(Solution::running_sum(vec![1, 2, 3, 4]), vec![1, 3, 6, 10]);
}

#[test]
fn test_running_sum2() {
    assert_eq!(Solution::running_sum(vec![1, 1, 1, 1, 1]), vec![1, 2, 3, 4, 5]);
}

#[test]
fn test_running_sum3() {
    assert_eq!(Solution::running_sum(vec![3, 1, 2, 10, 1]), vec![3, 4, 6, 16, 17]);
}
