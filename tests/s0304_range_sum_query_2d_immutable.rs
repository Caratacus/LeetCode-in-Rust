// Tests for Problem 0304: Range Sum Query 2D - Immutable
// Java reference: src/test/java/g0301_0400/s0304_range_sum_query_2d_immutable/NumMatrixTest.java

use leetcode_in_rust::s0304::range_sum_query_2d_immutable::NumMatrix;

#[test]
fn test_num_matrix() {
    let matrix = vec![
        vec![3, 0, 1, 4, 2],
        vec![5, 6, 3, 2, 1],
        vec![1, 2, 0, 1, 5],
        vec![4, 1, 0, 1, 7],
        vec![1, 0, 3, 0, 5],
    ];
    let mut num_matrix = NumMatrix::new(matrix);
    assert_eq!(num_matrix.sum_region(2, 1, 4, 3), 8);
    assert_eq!(num_matrix.sum_region(1, 1, 2, 2), 11);
    assert_eq!(num_matrix.sum_region(1, 2, 2, 4), 12);
}
