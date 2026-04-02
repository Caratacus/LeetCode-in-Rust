// Tests for Problem 1030: Matrix Cells in Distance Order
// Java reference: src/test/java/g1001_1100/s1030_matrix_cells_in_distance_order/SolutionTest.java

use leetcode_in_rust::s1030::matrix_cells_in_distance_order::Solution;

#[test]
fn test_all_cells_dist_order() {
    assert_eq!(
        Solution::all_cells_dist_order(1, 2, 0, 0),
        vec![vec![0, 0], vec![0, 1]]
    );
}

#[test]
fn test_all_cells_dist_order2() {
    assert_eq!(
        Solution::all_cells_dist_order(2, 2, 0, 1),
        vec![vec![0, 1], vec![0, 0], vec![1, 1], vec![1, 0]]
    );
}

#[test]
fn test_all_cells_dist_order3() {
    assert_eq!(
        Solution::all_cells_dist_order(2, 3, 1, 2),
        vec![vec![1, 2], vec![0, 2], vec![1, 1], vec![0, 1], vec![1, 0], vec![0, 0]]
    );
}
