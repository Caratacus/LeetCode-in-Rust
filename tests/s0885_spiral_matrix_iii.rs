// Tests for Problem 0885: Spiral Matrix III
// Java reference: src/test/java/g0801_0900/s0885_spiral_matrix_iii/SolutionTest.java

use leetcode_in_rust::s0885::spiral_matrix_iii::Solution;

#[test]
fn test_spiral_matrix_iii() {
    let result = Solution::spiral_matrix_iii(1, 4, 0, 0);
    assert_eq!(result, vec![vec![0, 0], vec![0, 1], vec![0, 2], vec![0, 3]]);
}

#[test]
fn test_spiral_matrix_iii2() {
    let result = Solution::spiral_matrix_iii(5, 6, 1, 4);
    assert_eq!(
        result,
        vec![
            vec![1, 4],
            vec![1, 5],
            vec![2, 5],
            vec![2, 4],
            vec![2, 3],
            vec![1, 3],
            vec![0, 3],
            vec![0, 4],
            vec![0, 5],
            vec![3, 5],
            vec![4, 5],
            vec![4, 4],
            vec![4, 3],
            vec![4, 2],
            vec![3, 2],
            vec![2, 2],
            vec![1, 2],
            vec![0, 2],
            vec![4, 1],
            vec![3, 1],
            vec![2, 1],
            vec![1, 1],
            vec![0, 1],
            vec![4, 0],
            vec![3, 0],
            vec![2, 0],
            vec![1, 0],
            vec![0, 0],
        ]
    );
}
