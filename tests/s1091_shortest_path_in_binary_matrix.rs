// Tests for Problem 1091: Shortest Path in Binary Matrix
// Java reference: src/test/java/g1001_1100/s1091_shortest_path_in_binary_matrix/SolutionTest.java

use leetcode_in_rust::s1091::shortest_path_in_binary_matrix::Solution;

#[test]
fn test_shortest_path_binary_matrix() {
    assert_eq!(Solution::shortest_path_binary_matrix(vec![vec![0, 1], vec![1, 0]]), 2);
}

#[test]
fn test_shortest_path_binary_matrix2() {
    assert_eq!(
        Solution::shortest_path_binary_matrix(vec![
            vec![0, 0, 0],
            vec![1, 1, 0],
            vec![1, 1, 0]
        ]),
        4
    );
}

#[test]
fn test_shortest_path_binary_matrix3() {
    assert_eq!(
        Solution::shortest_path_binary_matrix(vec![
            vec![1, 0, 0],
            vec![1, 1, 0],
            vec![1, 1, 0]
        ]),
        -1
    );
}
