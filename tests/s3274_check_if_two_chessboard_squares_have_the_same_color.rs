// Tests for Problem 3274: Check if Two Chessboard Squares Have the Same Color
// Java reference: src/test/java/g3201_3300/s3274_check_if_two_chessboard_squares_have_the_same_color/SolutionTest.java

use leetcode_in_rust::s3274::check_if_two_chessboard_squares_have_the_same_color::Solution;

#[test]
fn test_check_two_chessboards() {
    assert_eq!(Solution::check_two_chessboards("a1".to_string(), "c3".to_string()), true);
}

#[test]
fn test_check_two_chessboards2() {
    assert_eq!(Solution::check_two_chessboards("a1".to_string(), "h3".to_string()), false);
}
