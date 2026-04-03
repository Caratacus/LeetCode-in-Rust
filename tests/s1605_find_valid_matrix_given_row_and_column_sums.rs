// Tests for Problem 1605: Find Valid Matrix Given Row and Column Sums
// Java reference: src/test/java/g1601_1700/s1605_find_valid_matrix_given_row_and_column_sums/SolutionTest.java

use leetcode_in_rust::s1605::find_valid_matrix_given_row_and_column_sums::Solution;

#[test]
fn test_restore_matrix() {
    assert_eq!(
        Solution::restore_matrix(vec![3, 8], vec![4, 7]),
        vec![vec![3, 0], vec![1, 7]]
    );
}

#[test]
fn test_restore_matrix2() {
    assert_eq!(
        Solution::restore_matrix(vec![5, 7, 10], vec![8, 6, 8]),
        vec![vec![5, 0, 0], vec![3, 4, 0], vec![0, 2, 8]]
    );
}
