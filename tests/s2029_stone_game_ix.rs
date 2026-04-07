// Tests for Problem 2029: Stone Game IX
// Java reference: src/test/java/g2001_2100/s2029_stone_game_ix/SolutionTest.java

use leetcode_in_rust::s2029::stone_game_ix::Solution;

#[test]
fn test_stone_game_ix() {
    assert_eq!(Solution::stone_game_ix(vec![2, 1]), true);
}

#[test]
fn test_stone_game_ix2() {
    assert_eq!(Solution::stone_game_ix(vec![2]), false);
}

#[test]
fn test_stone_game_ix3() {
    assert_eq!(Solution::stone_game_ix(vec![5, 1, 2, 4, 3]), false);
}
