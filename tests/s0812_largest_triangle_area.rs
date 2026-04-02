// Tests for Problem 0812: Largest Triangle Area
// Java reference: src/test/java/g0801_0900/s0812_largest_triangle_area/SolutionTest.java

use leetcode_in_rust::s0812::largest_triangle_area::Solution;

#[test]
fn test_largest_triangle_area() {
    let result = Solution::largest_triangle_area(vec![
        vec![0, 0],
        vec![0, 1],
        vec![1, 0],
        vec![0, 2],
        vec![2, 0],
    ]);
    assert!((result - 2.0).abs() < 1e-6);
}

#[test]
fn test_largest_triangle_area2() {
    let result = Solution::largest_triangle_area(vec![vec![1, 0], vec![0, 0], vec![0, 1]]);
    assert!((result - 0.5).abs() < 1e-6);
}
