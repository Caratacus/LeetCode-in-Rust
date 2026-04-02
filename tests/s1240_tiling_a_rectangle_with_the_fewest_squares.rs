// Tests for Problem 1240: Tiling a Rectangle with the Fewest Squares
// Java reference: src/test/java/g1201_1300/s1240_tiling_a_rectangle_with_the_fewest_squares/SolutionTest.java

use leetcode_in_rust::s1240::tiling_a_rectangle_with_the_fewest_squares::Solution;

#[test]
fn test_tiling_rectangle() {
    assert_eq!(Solution::tiling_rectangle(2, 3), 3);
}

#[test]
fn test_tiling_rectangle2() {
    assert_eq!(Solution::tiling_rectangle(5, 8), 5);
}

#[test]
fn test_tiling_rectangle3() {
    assert_eq!(Solution::tiling_rectangle(11, 13), 6);
}
