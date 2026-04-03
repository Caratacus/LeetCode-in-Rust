// Tests for Problem 1697: Checking Existence of Edge Length Limited Paths
// Java reference: src/test/java/g1601_1700/s1697_checking_existence_of_edge_length_limited_paths/SolutionTest.java

use leetcode_in_rust::s1697::checking_existence_of_edge_length_limited_paths::Solution;

#[test]
fn test_distance_limited_paths_exist() {
    assert_eq!(
        Solution::distance_limited_paths_exist(
            3,
            vec![vec![0, 1, 2], vec![1, 2, 4], vec![2, 0, 8], vec![1, 0, 16]],
            vec![vec![0, 1, 2], vec![0, 2, 5]]
        ),
        vec![false, true]
    );
}

#[test]
fn test_distance_limited_paths_exist2() {
    assert_eq!(
        Solution::distance_limited_paths_exist(
            5,
            vec![vec![0, 1, 10], vec![1, 2, 5], vec![2, 3, 9], vec![3, 4, 13]],
            vec![vec![0, 4, 14], vec![1, 4, 13]]
        ),
        vec![true, false]
    );
}
