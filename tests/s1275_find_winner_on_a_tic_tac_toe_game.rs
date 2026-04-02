// Tests for Problem 1275: Find Winner on a Tic Tac Toe Game
// Java reference: src/test/java/g1201_1300/s1275_find_winner_on_a_tic_tac_toe_game/SolutionTest.java

use leetcode_in_rust::s1275::find_winner_on_a_tic_tac_toe_game::Solution;

#[test]
fn test_tictactoe() {
    assert_eq!(
        Solution::tictactoe(vec![vec![0, 0], vec![2, 0], vec![1, 1], vec![2, 1], vec![2, 2]]),
        "A"
    );
}

#[test]
fn test_tictactoe2() {
    assert_eq!(
        Solution::tictactoe(vec![
            vec![0, 0],
            vec![1, 1],
            vec![0, 1],
            vec![0, 2],
            vec![1, 0],
            vec![2, 0]
        ]),
        "B"
    );
}

#[test]
fn test_tictactoe3() {
    assert_eq!(
        Solution::tictactoe(vec![
            vec![0, 0],
            vec![1, 1],
            vec![2, 0],
            vec![1, 0],
            vec![1, 2],
            vec![2, 1],
            vec![0, 1],
            vec![0, 2],
            vec![2, 2]
        ]),
        "Draw"
    );
}
