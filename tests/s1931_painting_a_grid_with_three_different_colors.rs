// Tests for Problem 1931: Painting a Grid With Three Different Colors
// Java reference: src/test/java/g1901_2000/s1931_painting_a_grid_with_three_different_colors/SolutionTest.java

use leetcode_in_rust::s1931::painting_a_grid_with_three_different_colors::Solution;

#[test]
fn test_color_the_grid() {
    assert_eq!(Solution::color_the_grid(1, 1), 3);
}

#[test]
fn test_color_the_grid2() {
    assert_eq!(Solution::color_the_grid(1, 2), 6);
}

#[test]
fn test_color_the_grid3() {
    assert_eq!(Solution::color_the_grid(5, 5), 580986);
}
