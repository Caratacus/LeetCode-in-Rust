// Tests for Problem 1284: Minimum Number of Flips to Convert Binary Matrix to Zero Matrix
// Java reference: src/test/java/g1201_1300/s1284_minimum_number_of_flips_to_convert_binary_matrix_to_zero_matrix/SolutionTest.java

use leetcode_in_rust::s1284::minimum_number_of_flips_to_convert_binary_matrix_to_zero_matrix::Solution;

#[test]
fn test_min_flips() {
    assert_eq!(Solution::min_flips(vec![vec![0, 0], vec![0, 1]]), 3);
}

#[test]
fn test_min_flips2() {
    assert_eq!(Solution::min_flips(vec![vec![0]]), 0);
}

#[test]
fn test_min_flips3() {
    assert_eq!(Solution::min_flips(vec![vec![1, 0, 0], vec![1, 0, 0]]), -1);
}
