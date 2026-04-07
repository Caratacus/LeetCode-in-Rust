// Tests for Problem 2503: Maximum Number of Points From Grid Queries
// Java reference: src/test/java/g2401_2500/s2503_maximum_number_of_points_from_grid_queries/SolutionTest.java

use leetcode_in_rust::s2503::maximum_number_of_points_from_grid_queries::Solution;

#[test]
fn test_max_points() {
    assert_eq!(
        Solution::max_points(vec![vec![1, 2, 3], vec![2, 5, 7], vec![3, 5, 1]], vec![5, 6, 2]),
        vec![5, 8, 1]
    );
}

#[test]
fn test_max_points2() {
    assert_eq!(
        Solution::max_points(vec![vec![5, 2, 1], vec![1, 1, 2]], vec![3]),
        vec![0]
    );
}
