// Tests for Problem 2319: Check if Matrix Is X-Matrix
// Java reference: src/test/java/g2301_2400/s2319_check_if_matrix_is_x_matrix/SolutionTest.java

use leetcode_in_rust::s2319::check_if_matrix_is_x_matrix::Solution;

#[test]
fn test_check_x_matrix() {
    assert_eq!(
        Solution::check_x_matrix(vec![
            vec![2, 0, 0, 1],
            vec![0, 3, 1, 0],
            vec![0, 5, 2, 0],
            vec![4, 0, 0, 2]
        ]),
        true
    );
}

#[test]
fn test_check_x_matrix2() {
    assert_eq!(
        Solution::check_x_matrix(vec![vec![5, 7, 0], vec![0, 3, 1], vec![0, 5, 0]]),
        false
    );
}

#[test]
fn test_check_x_matrix3() {
    assert_eq!(
        Solution::check_x_matrix(vec![
            vec![0, 0, 0, 0, 1],
            vec![0, 4, 0, 1, 0],
            vec![0, 0, 5, 0, 0],
            vec![0, 5, 0, 2, 0],
            vec![4, 0, 0, 0, 2]
        ]),
        false
    );
}
