// Tests for Problem 2249: Count Lattice Points Inside a Circle
// Java reference: src/test/java/g2201_2300/s2249_count_lattice_points_inside_a_circle/SolutionTest.java

use leetcode_in_rust::s2249::count_lattice_points_inside_a_circle::Solution;

#[test]
fn test_count_lattice_points() {
    assert_eq!(Solution::count_lattice_points(vec![vec![2, 2, 1]]), 5);
}

#[test]
fn test_count_lattice_points2() {
    assert_eq!(Solution::count_lattice_points(vec![vec![2, 2, 2], vec![3, 4, 1]]), 16);
}
