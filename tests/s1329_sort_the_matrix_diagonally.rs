// Tests for Problem 1329: Sort the Matrix Diagonally
// Java reference: src/test/java/g1301_1400/s1329_sort_the_matrix_diagonally/SolutionTest.java

use leetcode_in_rust::s1329::sort_the_matrix_diagonally::Solution;

#[test]
fn test_diagonal_sort() {
    assert_eq!(
        Solution::diagonal_sort(vec![vec![3, 3, 1, 1], vec![2, 2, 1, 2], vec![1, 1, 1, 2]]),
        vec![vec![1, 1, 1, 1], vec![1, 2, 2, 2], vec![1, 2, 3, 3]]
    );
}

#[test]
fn test_diagonal_sort2() {
    assert_eq!(
        Solution::diagonal_sort(vec![
            vec![11, 25, 66, 1, 69, 7],
            vec![23, 55, 17, 45, 15, 52],
            vec![75, 31, 36, 44, 58, 8],
            vec![22, 27, 33, 25, 68, 4],
            vec![84, 28, 14, 11, 5, 50]
        ]),
        vec![
            vec![5, 17, 4, 1, 52, 7],
            vec![11, 11, 25, 45, 8, 69],
            vec![14, 23, 25, 44, 58, 15],
            vec![22, 27, 31, 36, 50, 66],
            vec![84, 28, 75, 33, 55, 68]
        ]
    );
}
