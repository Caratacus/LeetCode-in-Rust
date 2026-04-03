// Tests for Problem 1632: Rank Transform of a Matrix
// Java reference: src/test/java/g1601_1700/s1632_rank_transform_of_a_matrix/SolutionTest.java

use leetcode_in_rust::s1632::rank_transform_of_a_matrix::Solution;

#[test]
fn test_matrix_rank_transform() {
    assert_eq!(Solution::matrix_rank_transform(vec![vec![1, 2], vec![3, 4]]), vec![vec![1, 2], vec![2, 3]]);
}

#[test]
fn test_matrix_rank_transform2() {
    assert_eq!(Solution::matrix_rank_transform(vec![vec![7, 7], vec![7, 7]]), vec![vec![1, 1], vec![1, 1]]);
}

#[test]
fn test_matrix_rank_transform3() {
    assert_eq!(
        Solution::matrix_rank_transform(vec![
            vec![20, -21, 14],
            vec![-19, 4, 19],
            vec![22, -47, 24],
            vec![-19, 4, 19]
        ]),
        vec![
            vec![4, 2, 3],
            vec![1, 3, 4],
            vec![5, 1, 6],
            vec![1, 3, 4]
        ]
    );
}
