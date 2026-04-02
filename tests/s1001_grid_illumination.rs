// Tests for Problem 1001: Grid Illumination
// Java reference: src/test/java/g1001_1100/s1001_grid_illumination/SolutionTest.java

use leetcode_in_rust::s1001::grid_illumination::Solution;

#[test]
fn test_grid_illumination() {
    assert_eq!(
        Solution::grid_illumination(5, vec![vec![0, 0], vec![4, 4]], vec![vec![1, 1], vec![1, 0]]),
        vec![1, 0]
    );
}

#[test]
fn test_grid_illumination2() {
    assert_eq!(
        Solution::grid_illumination(5, vec![vec![0, 0], vec![4, 4]], vec![vec![1, 1], vec![1, 1]]),
        vec![1, 1]
    );
}

#[test]
fn test_grid_illumination3() {
    assert_eq!(
        Solution::grid_illumination(5, vec![vec![0, 0], vec![0, 4]], vec![vec![0, 4], vec![0, 1], vec![1, 4]]),
        vec![1, 1, 0]
    );
}
