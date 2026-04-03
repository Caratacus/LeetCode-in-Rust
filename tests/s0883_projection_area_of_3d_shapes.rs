// Tests for Problem 0883: Projection Area of 3D Shapes
// Java reference: src/test/java/g0801_0900/s0883_projection_area_of_3d_shapes/SolutionTest.java

use leetcode_in_rust::s0883::projection_area_of_3d_shapes::Solution;

#[test]
fn test_projection_area() {
    assert_eq!(Solution::projection_area(vec![vec![1, 2], vec![3, 4]]), 17);
}

#[test]
fn test_projection_area2() {
    assert_eq!(Solution::projection_area(vec![vec![2]]), 5);
}

#[test]
fn test_projection_area3() {
    assert_eq!(Solution::projection_area(vec![vec![1, 0], vec![0, 2]]), 8);
}
