// Tests for Problem 0240: Search a 2D Matrix II
// Java reference: src/test/java/g0201_0300/s0240_search_a_2d_matrix_ii/SolutionTest.java

use leetcode_in_rust::s0240::search_a_2d_matrix_ii::Solution;

#[test]
fn test_search_matrix() {
    let matrix = vec![
        vec![1, 4, 7, 11, 15],
        vec![2, 5, 8, 12, 19],
        vec![3, 6, 9, 16, 22],
        vec![10, 13, 14, 17, 24],
        vec![18, 21, 23, 26, 30],
    ];
    assert_eq!(Solution::search_matrix(matrix, 5), true);
}

#[test]
fn test_search_matrix2() {
    let matrix = vec![
        vec![1, 4, 7, 11, 15],
        vec![2, 5, 8, 12, 19],
        vec![3, 6, 9, 16, 22],
        vec![10, 13, 14, 17, 24],
        vec![18, 21, 23, 26, 30],
    ];
    assert_eq!(Solution::search_matrix(matrix, 20), false);
}
