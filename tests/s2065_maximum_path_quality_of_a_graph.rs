// Tests for Problem 2065: Maximum Path Quality of a Graph
// Java reference: src/test/java/g2001_2100/s2065_maximum_path_quality_of_a_graph/SolutionTest.java

use leetcode_in_rust::s2065::maximum_path_quality_of_a_graph::Solution;

#[test]
fn test_maximal_path_quality() {
    assert_eq!(
        Solution::maximal_path_quality(
            vec![0, 32, 10, 43],
            vec![vec![0, 1, 10], vec![1, 2, 15], vec![0, 3, 10]],
            49
        ),
        75
    );
}

#[test]
fn test_maximal_path_quality2() {
    assert_eq!(
        Solution::maximal_path_quality(
            vec![1, 2, 3, 4],
            vec![vec![0, 1, 10], vec![1, 2, 11], vec![2, 3, 12], vec![1, 3, 13]],
            50
        ),
        7
    );
}

#[test]
fn test_maximal_path_quality3() {
    assert_eq!(
        Solution::maximal_path_quality(
            vec![5, 10, 15, 20],
            vec![vec![0, 1, 10], vec![1, 2, 10], vec![0, 3, 10]],
            30
        ),
        25
    );
}
