// Tests for Problem 2013: Detect Squares
// Java reference: src/test/java/g2001_2100/s2013_detect_squares/DetectSquaresTest.java

use leetcode_in_rust::s2013::detect_squares::DetectSquares;

#[test]
fn test_detect_squares() {
    let mut detect_squares = DetectSquares::new();
    detect_squares.add(vec![3, 10]);
    detect_squares.add(vec![11, 2]);
    detect_squares.add(vec![3, 2]);
    assert_eq!(detect_squares.count(vec![11, 10]), 1);
    assert_eq!(detect_squares.count(vec![14, 8]), 0);
    detect_squares.add(vec![11, 2]);
    assert_eq!(detect_squares.count(vec![11, 10]), 2);
}
