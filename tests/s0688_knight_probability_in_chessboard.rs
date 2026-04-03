// Tests for Problem 0688: Knight Probability in Chessboard
// Java reference: src/test/java/g0601_0700/s0688_knight_probability_in_chessboard/SolutionTest.java

use leetcode_in_rust::s0688::knight_probability_in_chessboard::Solution;

#[test]
fn test_knight_probability() {
    let result = Solution::knight_probability(3, 2, 0, 0);
    assert!((result - 0.0625).abs() < 1e-5);
}

#[test]
fn test_knight_probability2() {
    let result = Solution::knight_probability(1, 0, 0, 0);
    assert!((result - 1.0).abs() < 1e-5);
}
