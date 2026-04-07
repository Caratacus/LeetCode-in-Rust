// Tests for Problem 2326: Spiral Matrix IV
// Java reference: src/test/java/g2301_2400/s2326_spiral_matrix_iv/SolutionTest.java

use leetcode_in_rust::s2326::spiral_matrix_iv::Solution;
use leetcode_in_rust::utils::linked_list_utils::linked_list_from_vec;

#[test]
fn test_spiral_matrix() {
    let head = linked_list_from_vec(vec![3, 0, 2, 6, 8, 1, 7, 9, 4, 2, 5, 5, 0]);
    assert_eq!(
        Solution::spiral_matrix(3, 5, head),
        vec![vec![3, 0, 2, 6, 8], vec![5, 0, -1, -1, 1], vec![5, 2, 4, 9, 7]]
    );
}

#[test]
fn test_spiral_matrix2() {
    let head = linked_list_from_vec(vec![0, 1, 2]);
    assert_eq!(
        Solution::spiral_matrix(1, 4, head),
        vec![vec![0, 1, 2, -1]]
    );
}
