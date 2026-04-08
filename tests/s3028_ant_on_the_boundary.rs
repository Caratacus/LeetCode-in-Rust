// Tests for Problem 3028: Ant on the Boundary
// Java reference: src/test/java/g3001_3100/s3028_ant_on_the_boundary/SolutionTest.java
use leetcode_in_rust::s3028::ant_on_the_boundary::Solution;
#[test]
fn test_return_to_boundary_count() {
    assert_eq!(Solution::return_to_boundary_count(vec![2, 3, -5]), 1);
}
#[test]
fn test_return_to_boundary_count2() {
    assert_eq!(Solution::return_to_boundary_count(vec![3, 2, -3, -4]), 0);
}
