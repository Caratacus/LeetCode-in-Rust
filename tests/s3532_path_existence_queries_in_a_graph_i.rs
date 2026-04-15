// Tests for Problem 3532: Path Existence Queries in a Graph I
// Java reference: src/test/java/g3501_3600/s3532_path_existence_queries_in_a_graph_i/SolutionTest.java

use leetcode_in_rust::s3532::path_existence_queries_in_a_graph_i::Solution;

#[test]
fn test_path_existence_queries() {
    assert_eq!(Solution::path_existence_queries(2, vec![1, 3], 1, vec![vec![0, 0], vec![0, 1]]), vec![true, false]);
}

#[test]
fn test_path_existence_queries2() {
    assert_eq!(Solution::path_existence_queries(4, vec![2, 5, 6, 8], 2, vec![vec![0, 1], vec![0, 2], vec![1, 3], vec![2, 3]]), vec![false, false, true, true]);
}
