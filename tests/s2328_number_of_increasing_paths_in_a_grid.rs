// Tests for Problem 2328: Number of Increasing Paths in a Grid
// Java reference: src/test/java/g2301_2400/s2328_number_of_increasing_paths_in_a_grid/SolutionTest.java

use leetcode_in_rust::s2328::number_of_increasing_paths_in_a_grid::Solution;

#[test]
fn test_count_paths() {
    assert_eq!(Solution::count_paths(vec![vec![1, 1], vec![3, 4]]), 8);
}

#[test]
fn test_count_paths2() {
    assert_eq!(Solution::count_paths(vec![vec![1], vec![2]]), 3);
}
