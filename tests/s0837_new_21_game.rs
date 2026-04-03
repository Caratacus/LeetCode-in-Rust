// Tests for Problem 0837: New 21 Game
// Java reference: src/test/java/g0801_0900/s0837_new_21_game/SolutionTest.java

use leetcode_in_rust::s0837::new_21_game::Solution;

#[test]
fn test_new21_game() {
    let result = Solution::new21_game(10, 1, 10);
    assert!((result - 1.0).abs() < 0.00001);
}

#[test]
fn test_new21_game2() {
    let result = Solution::new21_game(6, 1, 10);
    assert!((result - 0.6).abs() < 0.00001);
}

#[test]
fn test_new21_game3() {
    let result = Solution::new21_game(21, 17, 10);
    assert!((result - 0.73278).abs() < 0.00001);
}
