// Tests for Problem 3175: Find the First Player to Win K Games in a Row
// Java reference: src/test/java/g3101_3200/s3175_find_the_first_player_to_win_k_games_in_a_row/SolutionTest.java

use leetcode_in_rust::s3175::find_the_first_player_to_win_k_games_in_a_row::Solution;
#[test]
fn test_find_winning_player() {
    assert_eq!(Solution::find_winning_player(vec![4, 2, 6, 3, 9], 2), 2);
}
#[test]
fn test_find_winning_player2() {
    assert_eq!(Solution::find_winning_player(vec![2, 5, 4], 3), 1);
}
