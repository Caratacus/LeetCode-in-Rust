// Tests for Problem 3047: Find the Largest Area of Square Inside Two Rectangles
// Java reference: src/test/java/g3001_3100/s3047_find_the_largest_area_of_square_inside_two_rectangles/SolutionTest.java

use leetcode_in_rust::s3047::find_the_largest_area_of_square_inside_two_rectangles::Solution;

#[test]
fn test_largest_square_area() {
    let bottom_left = vec![vec![1, 1], vec![2, 2], vec![3, 1]];
    let top_right = vec![vec![3, 3], vec![4, 4], vec![6, 6]];
    assert_eq!(Solution::largest_square_area(bottom_left, top_right), 1);
}

#[test]
fn test_largest_square_area2() {
    let bottom_left = vec![vec![1, 1], vec![2, 2], vec![1, 2]];
    let top_right = vec![vec![3, 3], vec![4, 4], vec![3, 4]];
    assert_eq!(Solution::largest_square_area(bottom_left, top_right), 1);
}
