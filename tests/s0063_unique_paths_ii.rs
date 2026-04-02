// Tests for Problem 0063: Unique Paths II
// Java reference: src/test/java/g0001_0100/s0063_unique_paths_ii/SolutionTest.java

use leetcode_in_rust::s0063::unique_paths_ii::Solution;

#[test]
fn test_unique_paths_with_obstacles() {
    let grid = vec![vec![0, 0, 0], vec![0, 1, 0], vec![0, 0, 0]];
    assert_eq!(Solution::unique_paths_with_obstacles(grid), 2);
}

#[test]
fn test_unique_paths_with_obstacles2() {
    let grid = vec![vec![0, 1], vec![0, 0]];
    assert_eq!(Solution::unique_paths_with_obstacles(grid), 1);
}
