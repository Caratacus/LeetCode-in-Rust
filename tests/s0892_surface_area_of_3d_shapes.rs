// Tests for Problem 0892: Surface Area of 3D Shapes
// Java reference: src/test/java/g0801_0900/s0892_surface_area_of_3d_shapes/SolutionTest.java

use leetcode_in_rust::s0892::surface_area_of_3d_shapes::Solution;

#[test]
fn test_surface_area() {
    let result = Solution::surface_area(vec![vec![1, 2], vec![3, 4]]);
    assert_eq!(result, 34);
}

#[test]
fn test_surface_area2() {
    let result = Solution::surface_area(vec![vec![1, 1, 1], vec![1, 0, 1], vec![1, 1, 1]]);
    assert_eq!(result, 32);
}

#[test]
fn test_surface_area3() {
    let result = Solution::surface_area(vec![vec![2, 2, 2], vec![2, 1, 2], vec![2, 2, 2]]);
    assert_eq!(result, 46);
}
