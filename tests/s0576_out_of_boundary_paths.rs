// Tests for Problem 0576: Out of Boundary Paths
// Java reference: src/test/java/g0501_0600/s0576_out_of_boundary_paths/SolutionTest.java

use leetcode_in_rust::s0576::out_of_boundary_paths::Solution;

#[test]
fn test_find_paths() {
    assert_eq!(Solution::find_paths(2, 2, 2, 0, 0), 6);
}

#[test]
fn test_find_paths2() {
    assert_eq!(Solution::find_paths(1, 3, 3, 0, 1), 12);
}
