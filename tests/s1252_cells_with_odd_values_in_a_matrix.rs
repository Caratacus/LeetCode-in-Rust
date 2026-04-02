// Tests for Problem 1252: Cells with Odd Values in a Matrix
// Java reference: src/test/java/g1201_1300/s1252_cells_with_odd_values_in_a_matrix/SolutionTest.java

use leetcode_in_rust::s1252::cells_with_odd_values_in_a_matrix::Solution;

#[test]
fn test_odd_cells() {
    assert_eq!(Solution::odd_cells(2, 3, vec![vec![0, 1], vec![1, 1]]), 6);
}

#[test]
fn test_odd_cells2() {
    assert_eq!(Solution::odd_cells(2, 2, vec![vec![1, 1], vec![0, 0]]), 0);
}
