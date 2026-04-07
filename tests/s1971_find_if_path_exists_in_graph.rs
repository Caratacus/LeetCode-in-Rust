// Tests for Problem 1971: Find if Path Exists in Graph
// Java reference: src/test/java/g1901_2000/s1971_find_if_path_exists_in_graph/SolutionTest.java

use leetcode_in_rust::s1971::find_if_path_exists_in_graph::Solution;

#[test]
fn test_valid_path() {
    assert_eq!(
        Solution::valid_path(3, vec![vec![0, 1], vec![1, 2], vec![2, 0]], 0, 2),
        true
    );
}

#[test]
fn test_valid_path2() {
    assert_eq!(
        Solution::valid_path(
            6,
            vec![vec![0, 1], vec![0, 2], vec![3, 5], vec![5, 4], vec![4, 3]],
            0,
            5
        ),
        false
    );
}
