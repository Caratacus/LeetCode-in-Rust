// Tests for Problem 2038: Remove Colored Pieces if Both Neighbors are the Same Color
// Java reference: src/test/java/g2001_2100/s2038_remove_colored_pieces_if_both_neighbors_are_the_same_color/SolutionTest.java

use leetcode_in_rust::s2038::remove_colored_pieces_if_both_neighbors_are_the_same_color::Solution;

#[test]
fn test_winner_of_game() {
    assert_eq!(Solution::winner_of_game("AAABABB".to_string()), true);
}

#[test]
fn test_winner_of_game2() {
    assert_eq!(Solution::winner_of_game("AA".to_string()), false);
}

#[test]
fn test_winner_of_game3() {
    assert_eq!(Solution::winner_of_game("ABBBBBBBAAA".to_string()), false);
}
