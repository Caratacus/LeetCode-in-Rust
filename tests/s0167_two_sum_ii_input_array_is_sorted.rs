// Tests for Problem 0167: Two Sum II - Input Array Is Sorted
// Java reference: src/test/java/g0121_0200/s0167_two_sum_ii_input_array_is_sorted/SolutionTest.java

use leetcode_in_rust::s0167::two_sum_ii_input_array_is_sorted::Solution;

#[test]
fn test_two_sum() {
    assert_eq!(Solution::two_sum(vec![2, 7, 11, 15], 9), vec![1, 2]);
}

#[test]
fn test_two_sum2() {
    assert_eq!(Solution::two_sum(vec![2, 3, 4], 6), vec![1, 3]);
}

#[test]
fn test_two_sum3() {
    assert_eq!(Solution::two_sum(vec![-1, 0], -1), vec![1, 2]);
}
