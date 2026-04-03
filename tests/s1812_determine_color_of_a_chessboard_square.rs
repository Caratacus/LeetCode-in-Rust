// Tests for Problem 1812: Determine Color of a Chessboard Square
// Java reference: src/test/java/g1801_1900/s1812_determine_color_of_a_chessboard_square/SolutionTest.java

use leetcode_in_rust::s1812::determine_color_of_a_chessboard_square::Solution;

#[test]
fn test_square_is_white() {
    assert_eq!(Solution::square_is_white("a1".to_string()), false);
}

#[test]
fn test_square_is_white2() {
    assert_eq!(Solution::square_is_white("h3".to_string()), true);
}

#[test]
fn test_square_is_white3() {
    assert_eq!(Solution::square_is_white("c7".to_string()), false);
}
