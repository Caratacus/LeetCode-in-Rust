// Tests for Problem 2713: Maximum Strictly Increasing Cells in a Matrix
// Java reference: src/test/java/g2701_2800/s2713_maximum_strictly_increasing_cells_in_a_matrix/SolutionTest.java

use leetcode_in_rust::s2713::maximum_strictly_increasing_cells_in_a_matrix::Solution;

#[test]
fn test_max_increasing_cells() {
    assert_eq!(Solution::max_increasing_cells(vec![vec![3, 1], vec![3, 4]]), 2);
}

#[test]
fn test_max_increasing_cells2() {
    assert_eq!(Solution::max_increasing_cells(vec![vec![1, 1], vec![1, 1]]), 1);
}

#[test]
fn test_max_increasing_cells3() {
    assert_eq!(
        Solution::max_increasing_cells(vec![vec![3, 1, 6], vec![-9, 5, 7]]),
        4
    );
}
