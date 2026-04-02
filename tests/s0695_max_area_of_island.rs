// Tests for Problem 0695: Max Area of Island
// Java reference: src/test/java/g0601_0700/s0695_max_area_of_island/SolutionTest.java

use leetcode_in_rust::s0695::max_area_of_island::Solution;

#[test]
fn test_max_area_of_island() {
    let grid = vec![
        vec![0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0],
        vec![0, 1, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 1, 0, 0, 1, 1, 0, 0, 1, 0, 1, 0, 0],
        vec![0, 1, 0, 0, 1, 1, 0, 0, 1, 1, 1, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0],
    ];
    assert_eq!(Solution::max_area_of_island(grid), 6);
}

#[test]
fn test_max_area_of_island2() {
    let grid = vec![vec![0, 0, 0, 0, 0, 0, 0, 0]];
    assert_eq!(Solution::max_area_of_island(grid), 0);
}
