// Tests for Problem 3232: Find if Digit Game Can Be Won
// Java reference: src/test/java/g3201_3300/s3232_find_if_digit_game_can_be_won/SolutionTest.java

use leetcode_in_rust::s3232::find_if_digit_game_can_be_won::Solution;

#[test]
fn test_can_alice_win() {
    assert_eq!(Solution::can_alice_win(vec![1, 2, 3, 4, 10]), false);
}

#[test]
fn test_can_alice_win2() {
    assert_eq!(Solution::can_alice_win(vec![1, 2, 3, 4, 5, 14]), true);
}

#[test]
fn test_can_alice_win3() {
    assert_eq!(Solution::can_alice_win(vec![5, 5, 5, 25]), true);
}
