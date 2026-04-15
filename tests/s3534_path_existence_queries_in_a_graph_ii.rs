// Tests for Problem 3534: Path Existence Queries in a Graph II
// Java reference: src/test/java/g3501_3600/s3534_path_existence_queries_in_a_graph_ii/SolutionTest.java

use leetcode_in_rust::s3534::path_existence_queries_in_a_graph_ii::Solution;

#[test]
fn test_path_existence_queries() {
    assert_eq!(Solution::path_existence_queries(5, vec![1, 8, 3, 4, 2], 3, vec![vec![0, 3], vec![2, 4]]), vec![1, 1]);
}

#[test]
fn test_path_existence_queries2() {
    assert_eq!(Solution::path_existence_queries(5, vec![5, 3, 1, 9, 10], 2, vec![vec![0, 1], vec![0, 2], vec![2, 3], vec![4, 3]]), vec![1, 2, -1, 1]);
}

#[test]
fn test_path_existence_queries3() {
    assert_eq!(Solution::path_existence_queries(3, vec![3, 6, 1], 1, vec![vec![0, 0], vec![0, 1], vec![1, 2]]), vec![0, -1, -1]);
}
