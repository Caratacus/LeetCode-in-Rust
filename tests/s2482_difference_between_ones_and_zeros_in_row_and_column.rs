// Tests for Problem 2482: Difference Between Ones and Zeros in Row and Column
// Java reference: src/test/java/g2401_2500/s2482_difference_between_ones_and_zeros_in_row_and_column/SolutionTest.java

use leetcode_in_rust::s2482::difference_between_ones_and_zeros_in_row_and_column::Solution;

#[test]
fn test_ones_minus_zeros() {
    assert_eq!(
        Solution::ones_minus_zeros(vec![vec![0, 1, 1], vec![1, 0, 1], vec![0, 0, 1]]),
        vec![vec![0, 0, 4], vec![0, 0, 4], vec![-2, -2, 2]]
    );
}

#[test]
fn test_ones_minus_zeros2() {
    assert_eq!(
        Solution::ones_minus_zeros(vec![vec![1, 1, 1], vec![1, 1, 1]]),
        vec![vec![5, 5, 5], vec![5, 5, 5]]
    );
}
