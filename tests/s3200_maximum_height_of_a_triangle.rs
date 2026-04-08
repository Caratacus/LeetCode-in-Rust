// Tests for Problem 3200: Maximum Height of a Triangle
// Java reference: src/test/java/g3101_3200/s3200_maximum_height_of_a_triangle/SolutionTest.java

use leetcode_in_rust::s3200::maximum_height_of_a_triangle::Solution;

#[test]
fn test_max_height_of_triangle() {
    assert_eq!(Solution::max_height_of_triangle(2, 4), 3);
}

#[test]
fn test_max_height_of_triangle2() {
    assert_eq!(Solution::max_height_of_triangle(2, 1), 2);
}
