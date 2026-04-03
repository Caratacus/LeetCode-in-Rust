// Tests for Problem 1465: Maximum Area of a Piece of Cake After Horizontal and Vertical Cuts
// Java reference: src/test/java/g1401_1500/s1465_maximum_area_of_a_piece_of_cake_after_horizontal_and_vertical_cuts/SolutionTest.java

use leetcode_in_rust::s1465::maximum_area_of_a_piece_of_cake_after_horizontal_and_vertical_cuts::Solution;

#[test]
fn test_max_area() {
    assert_eq!(Solution::max_area(5, 4, vec![1, 2, 4], vec![1, 3]), 4);
}

#[test]
fn test_max_area2() {
    assert_eq!(Solution::max_area(5, 4, vec![3, 1], vec![1]), 6);
}

#[test]
fn test_max_area3() {
    assert_eq!(Solution::max_area(5, 4, vec![3], vec![3]), 9);
}
