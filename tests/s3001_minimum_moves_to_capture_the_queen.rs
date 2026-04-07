// Tests for Problem 3001: Minimum Moves to Capture the Queen
// Java reference: src/test/java/g3001_3100/s3001_minimum_moves_to_capture_the_queen/SolutionTest.java

use leetcode_in_rust::s3001::minimum_moves_to_capture_the_queen::Solution;

#[test]
fn test_min_moves_to_capture_the_queen() {
    assert_eq!(Solution::min_moves_to_capture_the_queen(1, 1, 8, 8, 2, 3), 2);
}

#[test]
fn test_min_moves_to_capture_the_queen2() {
    assert_eq!(Solution::min_moves_to_capture_the_queen(5, 3, 3, 4, 5, 2), 1);
}

#[test]
fn test_min_moves_to_capture_the_queen3() {
    assert_eq!(Solution::min_moves_to_capture_the_queen(1, 1, 3, 1, 5, 1), 2);
}

#[test]
fn test_min_moves_to_capture_the_queen4() {
    assert_eq!(Solution::min_moves_to_capture_the_queen(1, 1, 3, 3, 5, 5), 1);
}
